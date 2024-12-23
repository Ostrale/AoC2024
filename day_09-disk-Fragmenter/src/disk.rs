use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct File {
    pub id: u128,  // Unique identifier
    pub size: u32,  // Size in vector
    pub address: u128,  // Address in the disk, idx in the vector
}

#[derive(Clone)]
pub struct Disk {
    pub data: Vec<Option<u128>>,
}

impl Disk {
    pub fn from_string(s: &str) -> Self {
        let mut data = Vec::new();
        let mut id: u128 = 0;

        for (i, c) in s.chars().enumerate() {
            let nb = c.to_digit(10);
            let element_to_push: Option<u128>;
            if i % 2 == 0 {
                element_to_push = Option::Some(id);
                id += 1;
            } else {
                element_to_push = Option::None;
            }
            for _ in 0..nb.unwrap() {
                data.push(element_to_push);
            }
        }
        Disk { data }
    }

    pub fn _disk_representation(&self) -> String {
        self.data.iter().map(|x| match x {
            Some(id) => id.to_string(),
            None => ".".to_string(),
        }).collect()
    }

    pub fn find_free_space(&self, file: &File) -> Option<u128> {
        let mut free_space = 0;
        let mut free_space_start = 0;
        for (i, x) in self.data.iter().enumerate() {
            match x {
                Some(_) => {
                    free_space = 0;
                    free_space_start = i as u128;
                },
                None => {
                    free_space += 1;
                    if free_space == file.size {
                        return Some(free_space_start + 1);
                    }
                }
            }
            // if we reach the file address, we stop
            if i as u128 == file.address {
                return None;
            }
        }
        None
    }

    pub fn find_last_byte(&self, ignore_id: &Vec<u128>, start_idx: &mut usize) -> Option<File> {
        let mut add = 0;
        let mut good_id = 0;
        for (i, x) in self.data.iter().rev().enumerate().skip(*start_idx) {
            if x.is_none() {
                continue;
            }
            let id = x.unwrap();
            if ignore_id.contains(&id) {
                continue;
            }
            add = self.data.len() as u128 - i as u128 - 1;
            good_id = id;
            *start_idx = i + 1;
            break;
        }
        Some(File { id: good_id, size: 1, address: add })
    }

    pub fn find_last_file(&self, ignore_id: &Vec<u128>, start_idx: &mut usize) -> Option<File> {
        let mut idx_start = 0;
        let mut idx_end = 0;
        let mut founded_id: Option<u128> = None;
        for (i, x) in self.data.iter().rev().enumerate().skip(*start_idx) {
            if x.is_none() {
                continue;
            }
            let id = x.unwrap();
            if ignore_id.contains(&id) {
                continue;
            }
            if founded_id.is_none() {
                founded_id = Some(id);
                idx_start = i;
                idx_end = i;
            } else {
                if founded_id.unwrap() == id {
                    idx_start = i;
                } else {
                    *start_idx = i;
                    break;
                }
            }
        }
        if founded_id.is_none() {
            return None;
        }
        let size = idx_start - idx_end + 1;
        let last_file = File { id: founded_id.unwrap(), size: size as u32, address: self.data.len() as u128 - idx_start as u128 - 1 };
        Some(last_file)
    }

    pub fn move_file(&mut self, file: &File, new_address: u128) {
        // First delete the file at the current address
        let current_address = file.address;
        for i in current_address..current_address + file.size as u128 {
            self.data[i as usize] = None;
        }
        // Then move the file to the new address
        for i in new_address..new_address + file.size as u128 {
            self.data[i as usize] = Some(file.id);
        }
    }

    pub fn checksum(&self) -> u128 {
        let mut checksum = 0;
        for (i, x) in self.data.iter().enumerate() {
            if let Some(id) = x {
                checksum += i as u128 * id;
            }
        }
        checksum
    }
}

// Implémentation de Deref pour accéder aux méthodes de Vec
impl Deref for Disk {
    type Target = Vec<Option<u128>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// Implémentation de DerefMut pour permettre la modification du vecteur
impl DerefMut for Disk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

pub fn compact_disk(disk: &mut Disk) {
    let mut start_idx = 0;
    loop {
        let last_file = disk.find_last_byte(&vec![], &mut start_idx).unwrap();
        let free_space = disk.find_free_space(&last_file);
        if free_space.is_none() {
            break;
        }
        disk.move_file(&last_file, free_space.unwrap());
    }
}

pub fn compact_disk_v2(disk: &mut Disk) {
    let mut ignore_id = vec![];
    let last_file = disk.find_last_byte(&vec![], &mut 0).unwrap();
    let total_files = last_file.id;
    let mut start_idx = 0;
    for _i in 0..total_files {
        let last_file = disk.find_last_file(&ignore_id, &mut start_idx);
        if last_file.is_none() {
            break;
        }
        let last_file = last_file.unwrap();
        let free_space = disk.find_free_space(&last_file);
        if free_space.is_none() {
            ignore_id.push(last_file.id);
            continue;
        }
        disk.move_file(&last_file, free_space.unwrap());
        ignore_id.push(last_file.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_disk() {
        let input = "2333133121414131402";
        let mut disk = Disk::from_string(input);
        compact_disk(&mut disk);
        assert_eq!(disk.checksum(), 1928);
    }

    #[test]
    fn test_compact_disk_v2() {
        let input = "2333133121414131402";
        let mut disk = Disk::from_string(input);
        compact_disk_v2(&mut disk);
        assert_eq!(disk.checksum(), 2858);
    }
}