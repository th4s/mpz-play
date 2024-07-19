//! In this unit we want to run a simple connection test. Alice wants to open a TCP connection with
//! Bob that automatically de-/serializes sent/received messages. Bob is just listening on some
//! port and awaiting Alice's incoming connection.
//!
//! Luckily we already prepared some tooling to simplify the IO setup for you. In
//! [`common::tcp_connect`] you can find a function to easily set up said TCP connection.
//! Instantiating a  [`serio::codec::Bincode`]  and wrapping the TCP connection with
//! [serio::codec::Codec::new_framed] allows you to open a channel.
//!
//! To check that you set up the connection correctly, Alice should send some number to Bob. Bob
//! increments it by one and sends it back to Alice. For sending you can use [`serio::SinkExt::send`]
//! and for receiving it is [`serio::stream::IoStreamExt::expect_next`] which is both implemented by
//! the channel you just opened.
