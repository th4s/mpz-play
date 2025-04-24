//! In this unit we want to do an oblivious transfer (OT). Alice will be the OT sender and Bob will
//! be the OT receiver.
//!
//! We start again by opening a connection. To be able use the connection with our OT API you need
//! to wrap it in a [`mpz_common::io::Io`] and create a [`mpz_common::Context`] from it.
//!
//! Now either create an [`mpz_ot::OTSender`] or an [`mpz_ot::OTReceiver`].
//! You can use [`mpz_ot::chou_orlandi::Sender`] and [`mpz_ot::chou_orlandi::Receiver`] for this.
//! Then allocate a buffer for the messages. And perform the OT by calling [`mpz_ot::OTSender::queue_send_ot`]
//! or [`mpz_ot::OTReceiver::queue_recv_ot`]. Don't forget to flush the sender or receiver.
//! For creating messages that can be sent, you can use [`mpz_core::Block`].
