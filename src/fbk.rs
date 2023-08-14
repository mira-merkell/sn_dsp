use std::mem;

use crate::{
    alloc_buffer,
    bus::Bus,
    frame::Frame,
    node::Node,
    num::{
        zero,
        Float,
        Fp,
    },
};

#[derive(Debug)]
pub struct Single<T>(T);

impl<T> Single<T>
where
    T: Frame,
{
    #[must_use]
    pub fn new() -> Self {
        Self(zero())
    }
}

impl<T> Default for Single<T>
where
    T: Frame,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Frame> Node for Single<T> {
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            mem::swap(&mut self.0, frm);
        }
    }
}

#[derive(Debug)]
pub struct Del<T> {
    buffer: Box<[T]>,
    index:  usize,
}

impl<T> Del<T> {
    #[must_use]
    pub fn new(buffer: Box<[T]>) -> Self {
        Self {
            buffer,
            index: 0,
        }
    }

    #[must_use]
    pub fn buffer(&self) -> &[T] {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut [T] {
        &mut self.buffer
    }

    #[must_use]
    pub fn into_buffer(self) -> Box<[T]> {
        self.buffer
    }
}

impl<T> From<Box<[T]>> for Del<T> {
    fn from(value: Box<[T]>) -> Self {
        Self::new(value)
    }
}

impl<T> Del<T>
where
    T: Default,
{
    // Allocate memory for a new buffer on the heap
    #[must_use]
    pub fn alloc_new(size: usize) -> Self {
        Self::new(alloc_buffer(size))
    }
}

impl<T> Node for Del<T>
where
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            let y0 = self.buffer[self.index];
            self.buffer[self.index] = *frm;
            self.index += 1;
            if self.index == self.buffer.len() {
                self.index = 0;
            }
            *frm = y0;
        }
    }
}

#[derive(Debug)]
pub struct Fbk<'a, T> {
    feedback:     Fp,
    ss_del_frame: T,
    bus:          Bus<'a, T>,
}

impl<'a, T> Fbk<'a, T>
where
    T: Frame,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            feedback:     0.,
            ss_del_frame: zero(),
            bus:          Bus::new(),
        }
    }

    pub fn bus(&self) -> &Bus<'a, T> {
        &self.bus
    }

    pub fn bus_mut(&mut self) -> &mut Bus<'a, T> {
        &mut self.bus
    }

    pub fn into_bus(self) -> Bus<'a, T> {
        self.bus
    }

    #[must_use]
    pub fn with_feedback(value: Fp) -> Self {
        let mut fbk = Self::new();
        *fbk.feedback_mut() = value;
        fbk
    }

    pub fn feedback(&self) -> &Fp {
        &self.feedback
    }

    pub fn feedback_mut(&mut self) -> &mut Fp {
        &mut self.feedback
    }
}

impl<'a, T> Default for Fbk<'a, T>
where
    T: Frame,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Node for Fbk<'a, T>
where
    T: Frame,
{
    type Frame = T;

    fn proc(
        &mut self,
        frames: &mut [Self::Frame],
    ) {
        for frm in frames {
            let y = &mut [*frm + self.ss_del_frame * self.feedback.to_float()];
            self.bus.proc(y);
            self.ss_del_frame = y[0];
            *frm = y[0];
        }
    }
}
