use std::{marker::PhantomData, ops::{Deref, DerefMut}, ptr::NonNull};

#[derive(Clone, Copy, Debug)]
pub struct ScopedPtr<'a, T> {
    data: NonNull<T>,
    _marker: PhantomData<&'a T>
}

impl<'a, T> ScopedPtr<'a, T> {
    pub fn new(ptr: *const T) -> Option<Self> {
        Some(Self { data: NonNull::new(ptr as *mut T)?, _marker: PhantomData::default() })
    }

    pub unsafe fn new_unchecked(ptr: *const T) -> Self {
        Self { data: unsafe { NonNull::new_unchecked(ptr as *mut T) }, _marker: PhantomData::default() }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.data.as_ptr()
    }
}

impl<'a, T> From<&'a T> for ScopedPtr<'a, T> {
    fn from(value: &'a T) -> Self {
        Self { data: NonNull::from_ref(value), _marker: PhantomData::default() }
    }
}

impl<'a, T> Deref for ScopedPtr<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data.as_ptr() }
    }
}


#[derive(Debug)]
pub struct ScopedPtrMut<'a, T> {
    data: NonNull<T>,
    _marker: PhantomData<&'a mut T>
}

impl<'a, T> ScopedPtrMut<'a, T> {
    pub fn new(ptr: *mut T) -> Option<Self> {
        Some(Self { data: NonNull::new(ptr)?, _marker: PhantomData::default() })
    }

    pub fn new_unchecked(ptr: *mut T) -> Self {
        Self { data: unsafe { NonNull::new_unchecked(ptr) }, _marker: PhantomData::default() }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.data.as_ptr()
    }
}

impl<'a, T> From<&'a mut T> for ScopedPtrMut<'a, T> {
    fn from(value: &'a mut T) -> Self {
        Self { data: NonNull::from_mut(value), _marker: PhantomData::default() }
    }
}

impl<'a, T> Deref for ScopedPtrMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data.as_ptr() }
    }
}

impl<'a, T> DerefMut for ScopedPtrMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.data.as_ptr() }
    }
}