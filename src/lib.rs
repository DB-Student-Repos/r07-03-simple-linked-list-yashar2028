use std::iter::FromIterator;

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}


pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>> //There can be a head or not, start elemnt (option) | <T> since it may store any type | Using Box to allocate it on heap since list's size may change
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None} // Create a new instance of our list (empty), return type is SimpleLinkedList struct
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut current_node = &self.head; // Count the node and while there is some node move to the next node
        let mut count = 0;
        while let Some(node) = current_node { // Instead of while let we could write SimpleLinkedList::new() while !node.is_none()
            count += 1;
            current_node = &node.next;
        };
        
        count
    }
    pub fn push(&mut self, _element: T) {
        let new = Node { element: _element, next: self.head.take() }; // Insert the new element in its node, take head where the new head wil be the new inserted element 
        self.head = Some(Box::new(new)) // Now head will be the new element
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_some() {
            let head_node = self.head.take().unwrap(); // Take and unwarp the head (last element) and value is returned with Some(head_node.element) 
            self.head = head_node.next; // Next head after take and unwrap which will be the head_node.next
            Some(head_node.element) // Show the popped element
        } else {
             None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element) // it is accessing the head of our list, and if the head exists (it's not None), it returns a reference to the data stored in the head node
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();
        while let Some(x) = self.pop() {
            rev_list.push(x); // while there is some node pop them all and push them back (which will reverse the list) into the new empty defined list
        }
        rev_list
    }
}
impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut new_list = SimpleLinkedList::new();
        for item in _iter {
            new_list.push(item);
        }
        new_list
    } // from_iter() consumes an iterator and constructs a linked list (Self(Our list)) from its elements.
      // So it receives as an input, anything <T> that implements 'Iterator' trait
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut output_vec = Vec::new(); // Basically the reverse of the upper function: defines an implementation of the From trait for converting from a SimpleLinkedList<T> to a Vec<T>
        while let Some(item) = _linked_list.pop() {
            output_vec.push(item);
        }
        output_vec.reverse(); // Since popping reverse the order we reverse again to get back in the order
        output_vec
    }
}
