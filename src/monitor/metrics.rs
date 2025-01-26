use criterion::{
    measurement::{Measurement, ValueFormatter},
    Throughput,
};

use crate::metrics;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

type SharedRT = Arc<Mutex<Box<Option<Runtime>>>>;

pub struct StealOpsMeasurement {
    pub rt: SharedRT,
}

impl Measurement for StealOpsMeasurement {
    type Intermediate = SharedRT;
    type Value = u64;

    fn start(&self) -> Self::Intermediate {
        Arc::clone(&self.rt)
    }

    fn end(&self, i: Self::Intermediate) -> Self::Value {
        let mg = i.lock().unwrap();
        let rt_opt = mg.as_ref().as_ref();
        let rt = rt_opt.expect("None instead of runtime");
        metrics::total_steal_ops(rt)
    }

    fn add(&self, &v1: &Self::Value, &v2: &Self::Value) -> Self::Value {
        v1.checked_add(v2)
            .expect("overflow of Steal opreations in addition")
    }

    fn zero(&self) -> Self::Value {
        0
    }

    fn to_f64(&self, &val: &Self::Value) -> f64 {
        val as f64
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        &DurationFormatter
    }
}

/// criterion.rs copypast
pub(crate) struct DurationFormatter;
impl DurationFormatter {
    fn elements_per_second(&self, elems: f64, typical: f64, values: &mut [f64]) -> &'static str {
        let elems_per_second = elems * (1e9 / typical);
        let (denominator, unit) = if elems_per_second < 1000.0 {
            (1.0, " elem/s")
        } else if elems_per_second < 1000.0 * 1000.0 {
            (1000.0, "Kelem/s")
        } else if elems_per_second < 1000.0 * 1000.0 * 1000.0 {
            (1000.0 * 1000.0, "Melem/s")
        } else {
            (1000.0 * 1000.0 * 1000.0, "Gelem/s")
        };

        for val in values {
            let elems_per_second = elems * (1e9 / *val);
            *val = elems_per_second / denominator;
        }

        unit
    }
}
impl ValueFormatter for DurationFormatter {
    fn scale_throughputs(
        &self,
        typical: f64,
        throughput: &Throughput,
        values: &mut [f64],
    ) -> &'static str {
        match *throughput {
            Throughput::Bytes(_) | Throughput::BytesDecimal(_) => {
                panic!("assume elements for metrics results")
            }
            Throughput::Elements(elems) => self.elements_per_second(elems as f64, typical, values),
        }
    }

    fn scale_values(&self, ns: f64, values: &mut [f64]) -> &'static str {
        let (factor, unit) = if ns < 10f64.powi(0) {
            (10f64.powi(3), "ps")
        } else if ns < 10f64.powi(3) {
            (10f64.powi(0), "ns")
        } else if ns < 10f64.powi(6) {
            (10f64.powi(-3), "µs")
        } else if ns < 10f64.powi(9) {
            (10f64.powi(-6), "ms")
        } else {
            (10f64.powi(-9), "s")
        };

        for val in values {
            *val *= factor;
        }

        unit
    }

    fn scale_for_machines(&self, _values: &mut [f64]) -> &'static str {
        // no scaling is needed
        "ns"
    }
}
