use std::ffi::OsStr;

const DEFAULT_CAPACITY: usize = 392;

#[derive(Debug, PartialEq)]
pub struct StringBuilder {
    chars: Vec<char>
}

impl StringBuilder {
    ///
    /// Return a new `StringBuilder` with default initial capacity.
    pub fn new() -> StringBuilder {
        StringBuilder::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn ltrim(&mut self)-> &mut StringBuilder {
        let mut idx = self.len() - 1;
        let mut delete_from: Option<usize> = None;
        while idx >= 0 as usize  {
            if self.chars[idx] ==  32 as char //BLANK
                || self.chars[idx] == 9 as char // TAB
                || self.chars[idx] == 10 as char // LF
                || self.chars[idx] == 13 as char // CR
            {
                delete_from = Some(idx);
            } else {
                break;
            }
            
            if idx == 0 { break; }

            idx -= 1
        }
        
        if let Some(start) = delete_from {
            self.delete(start, self.len());
        }

        self
    }

    ///
    /// Return a new `StringBuilder` with an initial capacity.
    ///
    pub fn with_capacity(size: usize) -> StringBuilder {
        StringBuilder {
        chars: Vec::with_capacity(size),
        }
    }

    /// Add a type that can be viewed as a slice of bytes.
    ///
    /// # Example
    /// 
    /// use rstring_builder::StringBuilder;
    /// let mut builder = StringBuilder::new();
    /// builder.append("some string");
    /// 
    pub fn append<T: Vcharsable>(&mut self, buff: T) -> &mut StringBuilder {
        self.chars.append(buff.vechars().as_mut());
        self
    }

    /// Return the current length in chars of the underlying buffer.
    ///
    /// # Example
    ///
    /// use rstring_builder::StringBuilder;
    ///
    /// let mut builder = StringBuilder::new();
    /// builder.append("four");
    /// assert_eq!(builder.len(), 4);
    /// builder.append("華文");
    /// assert_eq!(builder.len(), 6);
    /// 
    pub fn len(&self) -> usize {
        self.chars.len()
    }

    /// Delete chars of index
    ///
    /// # Example
    ///
    /// use rstring_builder::StringBuilder;
    ///
    /// let mut builder = StringBuilder::new();
    /// builder.append("abc");
    /// assert_eq!("bc".to_string(), builder.delete_at(0).string());
    /// assert_eq!("b".to_string(), builder.delete_at(1).string());
    /// 
    pub fn delete_at(&mut self, start: usize) -> &mut StringBuilder {
        self.delete(start, start + 1)
    }

    /// Delete chars range
    ///
    /// # Example
    ///
    /// use rstring_builder::StringBuilder;
    ///
    /// let mut builder = StringBuilder::new();
    /// builder.append("abc\ndef");
    /// assert_eq!("adef".to_string(), builder.delete(1, 4).string());
    /// assert_eq!("".to_string(), builder.delete(0, builder.len()).string());
    /// 
    pub fn delete(&mut self, start: usize, end: usize) -> &mut StringBuilder {
        if end == 0 {
            panic!("end index must be greater then 0. end: {}", end);
        }
        if end <= start {
            panic!("End index must be greater than start. start: {} end: {}", start, end);
        }
        if end > self.chars.len() {
            panic!("Out of index range. end: {}", end);
        }
        for _i in start..end {
            self.chars.remove(start);
        }
        self
    }

    /// Clear string builder.
    ///
    /// # Example
    ///
    /// 
    /// use rstring_builder::StringBuilder;
    ///
    /// let mut builder = StringBuilder::new();
    /// builder.append("abc\ndef");
    /// assert_eq!("".to_string(), builder.clear().string());
    /// 
    pub fn clear(&mut self) -> &mut StringBuilder {
        self.chars.clear();
        self
    }

    /// Return String
    ///
    /// # Example
    /// 
    /// let mut builder = StringBuilder::new();
    /// builder.append("abc\ndef");
    /// assert_eq!("abc\ndef".to_string(), builder.string());
    /// 
    pub fn string(&self) -> String {
        self.chars.clone().into_iter().collect()
    }

    /// text builder is empty
    ///  # Example
    /// 
    /// use rstring_builder::StringBuilder;
    ///
    /// let mut builder = StringBuilder::new();
    /// assert_eq!(true, builder.is_empty());
    /// builder.append("abc\ndef");
    /// assert_eq!(false, builder.is_empty());
    /// 
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl ToString for StringBuilder {
    fn to_string(&self) -> String {
        self.string()
    }
}

pub trait Vcharsable {
    fn vechars(&self) -> Vec<char>;
}

impl Vcharsable for StringBuilder {
    fn vechars(&self) -> Vec<char> {
        self.string().chars().collect()
    }
}

impl Vcharsable for String {
    fn vechars(&self) -> Vec<char> {
        self.chars().collect()
    }
}

impl Vcharsable for OsStr {
    fn vechars(&self) -> Vec<char> {
        self.to_str().unwrap().chars().collect()
    }
}

impl<'a> Vcharsable for &'a str {
    fn vechars(&self) -> Vec<char> {
        self.chars().collect()
    }
}

impl Vcharsable for char {
    fn vechars(&self) -> Vec<char> {
        let mut vec = Vec::with_capacity(1);
        vec.push(*self);
        vec
    }
}

impl Vcharsable for bool {
    fn vechars(&self) -> Vec<char> {
        if *self {
        let mut vec = Vec::with_capacity(4);
        vec.push('t');
        vec.push('r');
        vec.push('u');
        vec.push('e');
        return vec;
        }

        let mut vec = Vec::with_capacity(5);
        vec.push('f');
        vec.push('a');
        vec.push('l');
        vec.push('s');
        vec.push('e');
        vec
    }
}

impl Vcharsable for usize {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}

impl Vcharsable for u8 {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}

impl Vcharsable for u16 {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}

impl Vcharsable for u32 {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}

impl Vcharsable for u64 {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}

impl Vcharsable for u128 {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}

impl Vcharsable for isize {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}

impl Vcharsable for i8 {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}

impl Vcharsable for i16 {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}

impl Vcharsable for i32 {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}

impl Vcharsable for i64 {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}

impl Vcharsable for i128 {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}

impl Vcharsable for f32 {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}

impl Vcharsable for f64 {
    fn vechars(&self) -> Vec<char> {
        let num_str: String = self.to_string();
        let mut vec = Vec::with_capacity(num_str.len());
        num_str.chars().for_each(|ch| vec.push(ch));
        vec
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ltrim() {
        let s = "hello world";
        let mut sql_builder = StringBuilder::new();
        sql_builder.append(s)
            .append(" ")
            .append('\n')
            .append(9 as char);
        let rst = sql_builder.ltrim().string();
        assert_eq!(s, &rst);
    }
}