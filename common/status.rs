use std::ops::Deref;

/// 循环内状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopStatus<T> {
    Break,
    Continue,
    Normal(T),
}

impl<T> LoopStatus<T> {
    /// 是否是normal
    /// 若为normal才会有数据
    pub fn is_normal(&self) -> bool {
        if let LoopStatus::Normal(_x) = &self {
            true
        } else {
            false
        }
    }
}

impl<T> Deref for LoopStatus<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if let LoopStatus::Normal(x) = &self {
            &x
        } else {
            panic!("not normal, can't deref");
        }
    }
}

impl<T> Default for LoopStatus<T> {
    fn default() -> Self {
        Self::Break
    }
}
