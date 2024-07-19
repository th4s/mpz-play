//! In this unit we want to do an oblivious transfer (OT). Alice will be the OT sender and Bob will
//! be the OT receiver.
//!
//! We start again by opening a connection. To be able use the connection with out OT API you need
//! to wrap it in a [`mpz_common::executor::STExecutor`], which will be the [`mpz_common::Context`].
//!
//! Now either create an [`mpz_ot::OTSender`] or an [`mpz_ot::OTReceiver`] and set it up by calling
//! [`mpz_ot::OTSetup::setup`]. You can use [`mpz_ot::chou_orlandi::Sender`] and
//! [`mpz_ot::chou_orlandi::Receiver`] for this. Then perform the OT by calling
//! [`mpz_ot::OTSender::send`] or [`mpz_ot::OTReceiver::receive`]. For creating messages that can
//! be sent, you can use [`mpz_core::Block`].
