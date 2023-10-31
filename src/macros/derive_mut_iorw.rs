macro_rules! derive_sync_mut_read {
    ($({$($lt:tt)*})? $ty:ty) => {
        impl $(<$($lt)*>)? ::std::io::Read for $ty {
            #[inline(always)]
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                (self as &Self).read(buf)
            }
            #[inline(always)]
            fn read_vectored(&mut self, bufs: &mut [::std::io::IoSliceMut<'_>]) -> ::std::io::Result<usize> {
                (self as &Self).read_vectored(bufs)
            }
            // read_to_end isn't here because this macro isn't supposed to be used on Chain-like
            // adapters
            // FUTURE is_read_vectored
        }
    };
}

macro_rules! derive_sync_mut_write {
    ($({$($lt:tt)*})? $ty:ty) => {
        impl $(<$($lt)*>)? ::std::io::Write for $ty {
            #[inline(always)]
            fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
                (self as &Self).write(buf)
            }
            #[inline(always)]
            fn flush(&mut self) -> ::std::io::Result<()> {
                (self as &Self).flush()
            }
            #[inline(always)]
            fn write_vectored(&mut self, bufs: &[::std::io::IoSlice<'_>]) -> ::std::io::Result<usize> {
                (self as &Self).write_vectored(bufs)
            }
            // FUTURE is_write_vectored
        }
    };
}

macro_rules! derive_sync_mut_rw {
    ($({$($lt:tt)*})? $ty:ty) => {
        forward_sync_read!($({$($lt)*})? $ty);
        forward_sync_write!($({$($lt)*})? $ty);
    };
}

macro_rules! derive_futures_mut_read {
    ($({$($lt:tt)*})? $ty:ty) => {
        impl $(<$($lt)*>)? ::futures_io::AsyncRead for $ty {
            #[inline(always)]
            fn poll_read(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                buf: &mut [u8],
            ) -> ::std::task::Poll<::std::io::Result<usize>> {
                ::std::pin::Pin::new(&mut &*self).poll_read(cx, buf)
            }
            #[inline(always)]
            fn poll_read_vectored(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                bufs: &mut [::std::io::IoSliceMut<'_>],
            ) -> ::std::task::Poll<::std::io::Result<usize>> {
                ::std::pin::Pin::new(&mut &*self).poll_read_vectored(cx, bufs)
            }
        }
    };
}
macro_rules! derive_futures_mut_write {
    ($({$($lt:tt)*})? $ty:ty) => {
        impl $(<$($lt)*>)? ::futures_io::AsyncWrite for $ty {
            #[inline(always)]
            fn poll_write(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                buf: &[u8],
            ) -> ::std::task::Poll<::std::io::Result<usize>> {
                ::std::pin::Pin::new(&mut &*self).poll_write(cx, buf)
            }
            #[inline(always)]
            fn poll_write_vectored(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                bufs: &[::std::io::IoSlice<'_>],
            ) -> ::std::task::Poll<::std::io::Result<usize>> {
                ::std::pin::Pin::new(&mut &*self).poll_write_vectored(cx, bufs)
            }
            #[inline(always)]
            fn poll_flush(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                ::std::pin::Pin::new(&mut &*self).poll_flush(cx)
            }
            #[inline(always)]
            fn poll_close(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                ::std::pin::Pin::new(&mut &*self).poll_close(cx)
            }
        }
    };
}

macro_rules! derive_futures_mut_rw {
    ($({$($lt:tt)*})? $ty:ty) => {
        derive_futures_mut_read!($({$($lt)*})? $ty);
        derive_futures_mut_write!($({$($lt)*})? $ty);
    };
}