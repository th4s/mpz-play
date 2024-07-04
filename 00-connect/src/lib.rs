//! In this unit we want to run a simple connection test. Alice wants to open a multiplexed TCP
//! connection with Bob that automatically de-/serializes sent/received messages. Bob is just
//! listening on some port and awaiting Alice's incoming connection.
//!
//! Luckily we already prepared some tooling to simplify the IO setup for you. In
//! [`common::tcp_mux`] you can find a function to easily set up said TCP connection. Running the
//! returned future with [`tokio::spawn`] in the background allows you to open a channel with
//! [`common::FramedUidMux::open_framed`] which is implemented by [`common::MuxControl`].
//!
//! To check that you set up the connection correctly, Alice should send some number to Bob. Bob
//! increments it by one and sends it back to Alice. For sending you can use [serio::SinkExt::send]
//! and for receiving it is [serio::stream::IoStreamExt::expect_next] which is both implemented by
//! the channel you just opened.
//!
//! After you are done you should make sure to properly close the connection on both sides. This
//! can be done by calling [`common::MuxControl::mux_mut`] and then [`common::YamuxCtrl::close`].
//! Also make sure in the end to await the join handle returned by [`tokio::spawn`].
