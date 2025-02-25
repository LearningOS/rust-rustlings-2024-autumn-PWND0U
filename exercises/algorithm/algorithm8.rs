/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T> {  
    q1: Queue<T>,  
    q2: Queue<T>,  
}  
  
impl<T> myStack<T> {  
    pub fn new() -> Self {  
        Self {  
            q1: Queue::new(),  
            q2: Queue::new(),  
        }  
    }  
  
    pub fn push(&mut self, elem: T) {  
        self.q1.enqueue(elem);  
    }  
  
    pub fn pop(&mut self) -> Result<T, &str> {  
        if self.q1.is_empty() {  
            return Err("Stack is empty");  
        }  
  
        // Move all elements from q1 to q2 except the last one  
        while self.q1.size() > 1 {  
            self.q2.enqueue(self.q1.dequeue().unwrap());  
        }  
  
        // The last element in q1 is the top of the stack  
        let top_element = self.q1.dequeue().unwrap();  
  
        // Swap the roles of q1 and q2  
        std::mem::swap(&mut self.q1, &mut self.q2);  
  
        // Clear q2 (now the former q1, which should be empty except for possible moves back)  
        while let Ok(_) = self.q2.dequeue() {}  
  
        Ok(top_element)  
    }  
  
    pub fn is_empty(&self) -> bool {  
        self.q1.is_empty()  
    }  
} 

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}