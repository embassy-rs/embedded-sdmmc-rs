#[cfg(feature = "log")]
macro_rules! sd_log {
    (trace, $($arg:expr),*) => { log::trace!($($arg),*); };
    (debug, $($arg:expr),*) => { log::debug!($($arg),*); };
    (warn, $($arg:expr),*) => { log::warn!($($arg),*); };
}

#[cfg(feature = "defmt")]
macro_rules! sd_log {
    (trace, $($arg:expr),*) => { defmt::trace!($($arg),*); };
    (debug, $($arg:expr),*) => { defmt::debug!($($arg),*); };
    (warn, $($arg:expr),*) => { defmt::warn!($($arg),*); };
}

#[cfg(not(any(feature = "log", feature = "defmt")))]
macro_rules! sd_log {
    ($level:ident, $($arg:expr),*) => { $( let _ = $arg; )* }
}
