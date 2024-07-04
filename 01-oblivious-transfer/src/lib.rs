//! In this unit we want to do an oblivious transfer (OT). Alice will be the OT sender and Bob will
//! be the OT receiver.
//!
//! We start again by opening a connection and polling it in the background, but this time there is
//! no need to open a channel. This is because we use an executor that abstracts the creation of
//! channels and threads for us. Simply create a new executor with
//! [mpz_common::executor::MTExecutor::new] by injecting a cloned [`common::MuxControl`] (so that
//! we still have one control to close the connection in the end). Then create a new thread with
//! [mpz_common::executor::MTExecutor::new_thread], which will be used as the context for all IO
//! between Alice and Bob.
//!
//! Now either create an [mpz_ot::OTSender] or an [mpz_ot::OTReceiver] and set it up by calling
//! [mpz_ot::OTSetup::setup]. You can use [mpz_ot::chou_orlandi::Sender] and
//! [mpz_ot::chou_orlandi::Receiver] for this. Then perform the OT by calling
//! [mpz_ot::OTSender::send] or [mpz_ot::OTReceiver::receive]. For creating messages that can be
//! sent, you can use [mpz_core::Block].
//!
//! In the end properly close the connection.
