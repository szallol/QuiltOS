//! proposed new Reader-Writer traits here until core gets them

use core::prelude::*;
use core::fmt;

pub trait Reader {
  type Err; // new associated error type

  // unchanged except for error type
  fn read(&mut self, buf: &mut [u8]) -> Result<uint, <Self as Reader>::Err>;

  // these all return partial results on error
  //fn read_to_end(&mut self) -> NonatomicResult<Vec<u8>, Vec<u8>, Err> { ... }
  //fn read_to_string(&self) -> NonatomicResult<String, Vec<u8>, Err> { ... }
  //fn read_at_least(&mut self, min: uint,  buf: &mut [u8]) -> NonatomicResult<uint, uint, Err>  { ... }
}

pub trait Writer {
  fn write(&mut self, buf: &[u8]) -> Result<uint, ()>;

  //fn write_all(&mut self, buf: &[u8]) -> NonatomicResult<(), uint, Err> { ... };


  fn write_fmt(&mut self, fmt: &fmt::Arguments) -> Result<(), ()> {
    // Create a shim which translates a Writer to a FormatWriter and saves
    // off I/O errors. instead of discarding them

    struct Adaptor<'a, T:'a> {
      inner: &'a mut T,
      error: Result<(), ()>
    }

    impl<'a, T: Writer> fmt::FormatWriter for Adaptor<'a, T> {
      fn write(&mut self, bytes: &[u8]) -> fmt::Result {
        match self.inner.write(bytes) {
          Ok(_) => Ok(()),
          Err(e) => {
            self.error = Err(e);
            Err(fmt::Error)
          }
        }
      }
    }


    let mut output = Adaptor { inner: self, error: Ok(()) };
    let _ = fmt::write(&mut output, fmt);
    output.error
  }

  fn flush(&mut self) -> Result<(), ()>;
}

/*
trait Writer {
  type Err;

  fn write(&mut self, buf: &[u8]) -> Result<uint, <Self as Writer>::Err>;

  //fn write_all(&mut self, buf: &[u8]) -> NonatomicResult<(), uint, Err> { ... };


  fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(),  <Self as Writer>::Err> {
    // Create a shim which translates a Writer to a FormatWriter and saves
    // off I/O errors. instead of discarding them

    struct Adaptor<'a, T> where T: Writer + 'a {
      inner: &'a mut T,
      error: Result<(), <T as Writer>::Err>,
    }

    impl<'a, T: Writer> fmt::FormatWriter for Adaptor<'a, T> {
      fn write(&mut self, bytes: &[u8]) -> fmt::Result {
        match self.inner.write(bytes) {
          Ok(()) => Ok(()),
          Err(e) => {
            self.error = Err(e);
            Err(fmt::Error)
          }
        }
      }
    }


    let mut output = Adaptor { inner: self, error: Ok(()) };
    match fmt::write(&mut output, fmt) {
      Ok(()) => Ok(()),
      Err(..) => output.error
    }
  }

  //fn flush(&mut self) -> Result<(), Err> { ... }
}
*/
