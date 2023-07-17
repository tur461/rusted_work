use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{ready, Context, Poll};

use crate::cipher::Cipher;
use crate::util::eof;

use futures_util::AsyncRead;

enum State {
    Iv,
    Read,
    Done,
}

pub struct DecryptReadExact<'a, A: ?Sized> {
    cipher: Arc<Mutex<Cipher>>,
    reader: &'a mut A,
    buf: &'a mut [u8],
    pos: usize,
    state: State,
}

pub fn read_exact<'a, A>(
    cipher: Arc<Mutex<Cipher>>,
    reader: &'a mut A,
    buf: &'a mut [u8],
) -> DecryptReadExact<'a, A>
where
    A: AsyncRead + Unpin + ?Sized,
{
    DecryptReadExact {
        cipher,
        reader,
        buf,
        pos: 0,
        state: State::Iv,
    }
}

impl<A> Future for DecryptReadExact<'_, A>
where
    A: AsyncRead + Unpin + ?Sized,
{
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        let me = &mut *self;
        loop {
            match &mut me.state {
                State::Iv => {
                    let mut cipher = me.cipher.lock().unwrap();
                    if cipher.is_decrypt_inited() {
                        me.state = State::Read;
                        continue;
                    }
                    while me.pos < cipher.iv_len() {
                        let n =
                            ready!(Pin::new(&mut *me.reader)
                                .poll_read(cx, &mut cipher.iv_mut()[me.pos..]))?;
                        me.pos += n;
                        if n == 0 {
                            return Err(eof()).into();
                        }
                    }
                    me.pos = 0;
                    cipher.init_decrypt();
                    me.state = State::Read;
                }
                State::Read => {
                    // if our buffer is empty, then we need to read some data to continue.
                    while me.pos < me.buf.len() {
                        let n =
                            ready!(Pin::new(&mut *me.reader).poll_read(cx, &mut me.buf[me.pos..]))?;
                        me.pos += n;
                        if n == 0 {
                            return Err(eof()).into();
                        }
                    }
                    let copy_len = me.buf.len();
                    let mut cipher = me.cipher.lock().unwrap();
                    cipher.decrypt(&mut me.buf[..copy_len]);
                    me.state = State::Done;
                }
                State::Done => {
                    return Poll::Ready(Ok(me.pos));
                }
            }
        }
    }
}
