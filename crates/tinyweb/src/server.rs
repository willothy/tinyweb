use tinyweb_core::{
    body::Body,
    error::{ServeConnectionError, ServeError},
    incoming::Incoming,
    io::Io,
    maybe_send::MaybeSend,
    runtime::Runtime,
    service::Service,
};

use crate::io::TokioIo;

pub async fn serve_connection<IO, S, R>(
    io: IO,
    service: S,
    runtime: R,
) -> Result<(), ServeConnectionError>
where
    IO: Io,
    S: Service,
    R: Runtime,
{
    let mut conn = h2::server::handshake(TokioIo::new(io))
        .await
        .map_err(ServeConnectionError::Handshake)?;

    while let Some(result) = conn.accept().await {
        let (req, respond) = result.map_err(ServeConnectionError::Accept)?;
        let service = service.clone();

        runtime.spawn(Box::pin(async move {
            let response = service.call(req).await;
            let _ = write_response(response, respond).await;
        }));
    }

    Ok(())
}

pub async fn serve<I, S, R>(
    mut incoming: I,
    service: S,
    runtime: R,
) -> Result<(), ServeError<I::Error>>
where
    I: Incoming,
    S: Service,
    R: Runtime,
    I::Error: MaybeSend + 'static,
{
    loop {
        let (io, _addr) = incoming.accept().await.map_err(ServeError::Accept)?;

        let service = service.clone();
        let runtime_for_conn = runtime.clone();

        runtime.spawn(Box::pin(async move {
            let _ = serve_connection(io, service, runtime_for_conn).await;
        }));
    }
}

async fn write_response(
    response: http::Response<Body>,
    mut respond: h2::server::SendResponse<bytes::Bytes>,
) -> Result<(), h2::Error> {
    let (parts, body) = response.into_parts();
    let head = http::Response::from_parts(parts, ());

    match body {
        Body::Empty => {
            respond.send_response(head, true)?;
        }
        Body::Data(data) => {
            if data.is_empty() {
                respond.send_response(head, true)?;
            } else {
                let mut send = respond.send_response(head, false)?;
                send_bytes(&mut send, data, true).await?;
            }
        }
        Body::Stream(mut stream) => {
            let mut send = respond.send_response(head, false)?;
            loop {
                let item = core::future::poll_fn(|cx| stream.as_mut().poll_next(cx)).await;
                match item {
                    Some(Ok(chunk)) => {
                        if !chunk.is_empty() {
                            send_bytes(&mut send, chunk, false).await?;
                        }
                    }
                    Some(Err(_)) => {
                        send.send_reset(h2::Reason::INTERNAL_ERROR);
                        return Ok(());
                    }
                    None => {
                        send.send_data(bytes::Bytes::new(), true)?;
                        return Ok(());
                    }
                }
            }
        }
    }

    Ok(())
}

async fn send_bytes(
    send: &mut h2::SendStream<bytes::Bytes>,
    mut data: bytes::Bytes,
    end_of_stream: bool,
) -> Result<(), h2::Error> {
    while !data.is_empty() {
        send.reserve_capacity(data.len());

        let cap = match core::future::poll_fn(|cx| send.poll_capacity(cx)).await {
            Some(Ok(n)) => n,
            Some(Err(e)) => return Err(e),
            None => return Ok(()),
        };

        let chunk = data.split_to(cap.min(data.len()));
        let eos = data.is_empty() && end_of_stream;
        send.send_data(chunk, eos)?;
    }

    Ok(())
}
