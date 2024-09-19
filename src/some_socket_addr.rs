#[allow(unused_imports)]
use std::{fmt::Display, net::SocketAddr, sync::Arc};

/// Some form of accepted connection's address.
/// Variant depends on variant used in [`ListenerAddress`].
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SomeSocketAddr {
    Tcp(SocketAddr),
    #[cfg(all(feature = "unix", unix))]
    #[cfg_attr(docsrs_alt, doc(cfg(all(feature = "unix", unix))))]
    Unix(tokio::net::unix::SocketAddr),
    #[cfg(feature = "inetd")]
    #[cfg_attr(docsrs_alt, doc(cfg(feature = "inetd")))]
    Stdio,
    #[cfg(all(feature = "vsock", target_os = "linux"))]
    #[cfg_attr(docsrs_alt, doc(cfg(all(feature = "vsock", target_os = "linux"))))]
    Vsock(tokio_vsock::VsockAddr),
    #[cfg(feature = "multi-listener")]
    #[cfg_attr(docsrs_alt, doc(cfg(feature = "multi-listener")))]
    Multiple,
}

impl Display for SomeSocketAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SomeSocketAddr::Tcp(x) => x.fmt(f),
            #[cfg(all(feature = "unix", unix))]
            SomeSocketAddr::Unix(_x) => "unix".fmt(f),
            #[cfg(feature = "inetd")]
            SomeSocketAddr::Stdio => "stdio".fmt(f),
            #[cfg(all(feature = "vsock", target_os = "linux"))]
            SomeSocketAddr::Vsock(x) => x.fmt(f),
            #[cfg(feature = "multi-listener")]
            SomeSocketAddr::Multiple => "multiple".fmt(f),
        }
    }
}

impl SomeSocketAddr {
    /// Convert this address representation into a clonable form.
    /// For UNIX socket addresses, it converts them to a string using Debug representation.
    #[must_use]
    pub fn clonable(self) -> SomeSocketAddrClonable {
        match self {
            SomeSocketAddr::Tcp(x) => SomeSocketAddrClonable::Tcp(x),
            #[cfg(all(feature = "unix", unix))]
            SomeSocketAddr::Unix(x) => SomeSocketAddrClonable::Unix(Arc::new(x)),
            #[cfg(feature = "inetd")]
            SomeSocketAddr::Stdio => SomeSocketAddrClonable::Stdio,
            #[cfg(all(feature = "vsock", target_os = "linux"))]
            SomeSocketAddr::Vsock(x) => SomeSocketAddrClonable::Vsock(x),
            #[cfg(feature = "multi-listener")]
            SomeSocketAddr::Multiple => SomeSocketAddrClonable::Multiple,
        }
    }
}

/// Other representation of [`SomeSocketAddr`] with Arc-wrapped Unix addresses to enable cloning
#[derive(Debug, Clone)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SomeSocketAddrClonable {
    Tcp(SocketAddr),
    #[cfg(all(feature = "unix", unix))]
    #[cfg_attr(docsrs_alt, doc(cfg(all(feature = "unix", unix))))]
    Unix(Arc<tokio::net::unix::SocketAddr>),
    #[cfg(feature = "inetd")]
    #[cfg_attr(docsrs_alt, doc(cfg(feature = "inetd")))]
    Stdio,
    #[cfg(all(feature = "vsock", target_os = "linux"))]
    #[cfg_attr(docsrs_alt, doc(cfg(all(feature = "vsock", target_os = "linux"))))]
    Vsock(tokio_vsock::VsockAddr),
    #[cfg(feature = "multi-listener")]
    #[cfg_attr(docsrs_alt, doc(cfg(feature = "multi-listener")))]
    Multiple,
}

impl Display for SomeSocketAddrClonable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SomeSocketAddrClonable::Tcp(x) => x.fmt(f),
            #[cfg(all(feature = "unix", unix))]
            SomeSocketAddrClonable::Unix(x) => write!(f, "unix:{x:?}"),
            #[cfg(feature = "inetd")]
            SomeSocketAddrClonable::Stdio => "stdio".fmt(f),
            #[cfg(all(feature = "vsock", target_os = "linux"))]
            SomeSocketAddrClonable::Vsock(x) => x.fmt(f),
            #[cfg(feature = "multi-listener")]
            SomeSocketAddrClonable::Multiple => "multiple".fmt(f),
        }
    }
}

impl From<SomeSocketAddr> for SomeSocketAddrClonable {
    fn from(value: SomeSocketAddr) -> Self {
        value.clonable()
    }
}
