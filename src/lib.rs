#![no_std]

pub trait FallibleStreamingIterator {
    type Item: ?Sized;
    type Error;

    fn advance(&mut self) -> Result<(), Self::Error>;

    fn get(&self) -> Option<&Self::Item>;

    #[inline]
    fn next(&mut self) -> Result<Option<&Self::Item>, Self::Error> {
        self.advance()?;
        Ok(self.get())
    }
}
