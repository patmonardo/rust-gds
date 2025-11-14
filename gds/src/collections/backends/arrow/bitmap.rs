use arrow2::bitmap::Bitmap;

/// Lightweight owner for Arrow validity bitmaps.
#[derive(Clone, Debug)]
pub struct ArrowBitmap {
    inner: Bitmap,
}

impl ArrowBitmap {
    pub fn from_bitmap(inner: Bitmap) -> Self {
        Self { inner }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_set(&self, index: usize) -> bool {
        self.inner.get(index).unwrap_or(false)
    }

    pub fn bitmap(&self) -> &Bitmap {
        &self.inner
    }
}
