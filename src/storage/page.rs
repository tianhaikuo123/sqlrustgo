// src/storage/page.rs 
// 页结构：固定大小的数据页 

pub const PAGE_SIZE: usize = 4096; // 4KB 页大小 

/// 页ID类型 
pub type PageId = u32; 

/// 数据页 
#[derive(Debug, Clone)] 
pub struct Page { 
    pub page_id: PageId,           // 页ID 
    pub data: [u8; PAGE_SIZE],     // 页数据 
    pub is_dirty: bool,            // 是否被修改（需要写回磁盘） 
    pub pin_count: u32,            // 引脚计数（被引用的次数） 
} 

impl Page { 
    /// 创建新页 
    pub fn new(page_id: PageId) -> Self { 
        Page { 
            page_id, 
            data: [0; PAGE_SIZE], 
            is_dirty: false, 
            pin_count: 0, 
        } 
    } 

    /// 从字节数组创建页 
    pub fn from_bytes(page_id: PageId, bytes: &[u8]) -> Self { 
        let mut data = [0; PAGE_SIZE]; 
        let len = bytes.len().min(PAGE_SIZE); 
        data[..len].copy_from_slice(&bytes[..len]); 
        Page { 
            page_id, 
            data, 
            is_dirty: false, 
            pin_count: 0, 
        } 
    } 

    /// 获取页数据 
    pub fn get_data(&self) -> &[u8] { 
        &self.data 
    } 

    /// 写入数据到页 
    pub fn write(&mut self, offset: usize, bytes: &[u8]) -> Result<(), String> { 
        if offset + bytes.len() > PAGE_SIZE { 
            return Err("Write out of bounds".to_string()); 
        } 
        self.data[offset..offset + bytes.len()].copy_from_slice(bytes); 
        self.is_dirty = true; 
        Ok(()) 
    } 

    /// 读取页数据 
    pub fn read(&self, offset: usize, len: usize) -> Result<&[u8], String> { 
        if offset + len > PAGE_SIZE { 
            return Err("Read out of bounds".to_string()); 
        } 
        Ok(&self.data[offset..offset + len]) 
    } 

    /// 增加引脚计数 
    pub fn pin(&mut self) { 
        self.pin_count += 1; 
    } 

    /// 减少引脚计数 
    pub fn unpin(&mut self) { 
        if self.pin_count > 0 { 
            self.pin_count -= 1; 
        } 
    } 
} 

#[cfg(test)] 
mod tests { 
    use super::*; 

    #[test] 
    fn test_page_creation() { 
        let page = Page::new(1); 
        assert_eq!(page.page_id, 1); 
        assert!(!page.is_dirty); 
        assert_eq!(page.pin_count, 0); 
    } 

    #[test] 
    fn test_page_write_read() { 
        let mut page = Page::new(1); 
        let test_data = b"Hello, SQLRustGo!"; 
        
        page.write(0, test_data).unwrap(); 
        let read_data = page.read(0, test_data.len()).unwrap(); 
        
        assert_eq!(read_data, test_data); 
        assert!(page.is_dirty); 
    } 

    #[test] 
    fn test_page_pin() { 
        let mut page = Page::new(1); 
        page.pin(); 
        assert_eq!(page.pin_count, 1); 
        page.unpin(); 
        assert_eq!(page.pin_count, 0); 
    } 
}
