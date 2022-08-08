use crate as burn;

use crate::module::Module;
use crate::module::{Forward, Param};
use crate::tensor::back::Backend;
use crate::tensor::{Distribution, Shape, Tensor};
use std::ops::Deref;

pub struct LinearConfig {
    d_input: usize,
    d_output: usize,
    bias: bool,
}

#[derive(Module, Debug)]
pub struct Linear<B>
where
    B: Backend,
{
    weight: Param<Tensor<2, B>>,
    bias: Param<Option<Tensor<1, B>>>,
}

impl<B: Backend> Linear<B> {
    pub fn new(config: &LinearConfig) -> Self {
        let weight = Tensor::random(
            Shape::new([config.d_input, config.d_output]),
            Distribution::Standard,
        );
        let bias = match config.bias {
            true => Some(Tensor::zeros(Shape::new([config.d_output]))),
            false => None,
        };

        Self {
            weight: Param::new(weight),
            bias: Param::new(bias),
        }
    }
}

impl<B: Backend, const D: usize> Forward<&Tensor<D, B>, Tensor<D, B>> for Linear<B> {
    fn forward(&self, input: &Tensor<D, B>) -> Tensor<D, B> {
        let output = self.weight.unsqueeze().matmul(input);

        match self.bias.deref() {
            Some(bias) => output + bias.unsqueeze(),
            None => output,
        }
    }
}