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

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    #[inline]
    fn all<F>(&mut self, mut f: F) -> Result<bool, Self::Error>
        where Self: Sized,
              F: FnMut(&Self::Item) -> bool
    {
        while let Some(e) = self.next()? {
            if !f(e) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    #[inline]
    fn any<F>(&mut self, mut f: F) -> Result<bool, Self::Error>
        where Self: Sized,
              F: FnMut(&Self::Item) -> bool
    {
        self.all(|e| !f(e)).map(|r| !r)
    }

    #[inline]
    fn by_ref(&mut self) -> &mut Self
        where Self: Sized
    {
        self
    }

    #[inline]
    fn count(mut self) -> Result<usize, Self::Error>
        where Self: Sized
    {
        let mut count = 0;
        while let Some(_) = self.next()? {
            count += 1;
        }
        Ok(count)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn _is_object_safe(_: &FallibleStreamingIterator<Item = (), Error = ()>) {}
}
