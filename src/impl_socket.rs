use addin1c::{AddinResult, Variant};
use smallvec::SmallVec;
use zmq::{Message, PollEvents, Socket, SNDMORE};

pub fn send(socket: &Socket, data: &mut Variant) -> AddinResult {
    let data = data.get_blob()?;
    socket.send(data, 0)?;
    Ok(())
}

pub fn send_part(socket: &Socket, data: &mut Variant) -> AddinResult {
    let data = data.get_blob()?;
    socket.send(data, SNDMORE)?;
    Ok(())
}

pub fn recv(
    socket: &Socket,
    timeout: &mut Variant,
    msg: &mut Message,
    ret_value: &mut Variant,
) -> AddinResult {
    let timeout = timeout.get_i32()? as i64;
    if socket.poll(PollEvents::POLLIN, timeout)? != 1 {
        return Ok(());
    }

    socket.recv(msg, zmq::DONTWAIT)?;
    ret_value.set_blob(msg)?;

    Ok(())
}

pub fn recv_multipart(
    socket: &Socket,
    parts: &mut SmallVec<[Message; 4]>,
    timeout: &mut Variant,
    ret_value: &mut Variant,
) -> AddinResult {
    let timeout = timeout.get_i32()? as i64;
    if socket.poll(PollEvents::POLLIN, timeout)? != 1 {
        return Ok(());
    }

    let mut count = 0;
    loop {
        if count + 1 > parts.len() {
            parts.push(Message::new());
        }
        socket.recv(&mut parts[count], zmq::DONTWAIT)?;
        count += 1;
        if !socket.get_rcvmore()? {
            break;
        }
    }

    parts.truncate(count);
    ret_value.set_i32(count as _);

    Ok(())
}

pub fn get_part(
    parts: &SmallVec<[Message; 4]>,
    part: &mut Variant<'_>,
    ret_value: &mut Variant<'_>,
) -> AddinResult {
    let part = part.get_i32()? as usize;
    if part >= parts.len() {
        return Err("Part does not exist".into());
    }
    ret_value.set_blob(&parts[part])?;
    Ok(())
}
