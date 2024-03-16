use crate::domain::entry::Entry;
use crate::domain::index_file::AppendInfo;
use crate::infrastructure::event::EventInfo;



//////////////////////
/// File域事件类型
/////////////////////
#[derive(Clone, Debug)]
pub enum ActiveFileEventType {
    // 创建ActiveFile成功
    CreateSuccess,
    // 创建ActiveFile失败
    CreateFail,
    // 成功追加一条数据
    AppendSuccess,
    // ActiveFile已经达到最大可写上线(需要转换为OldFile)
    OutOfWrite,
}


#[derive(Clone,Debug)]
pub enum ActiveFileEventData {
    // 创建ActiveFile成功,ActiveFile名称
    CreateSuccess(String),
    // 创建ActiveFile失败，失败原因的描述
    CreateFail(String),
    // 写入成功之后返回起索引的序列化对象(KeyPair)
    AppendSuccess(AppendInfo),
    // 当ActiveFile写入数据失败的时候，返回的数据(active_name,write_entity)
    OutOfWrite(String, Entry),

}



/*
    ActiveFile可能会引起的事件
*/
#[derive(Clone,Debug)]
pub struct ActiveFileEvent {
    event_info: EventInfo,
    event_type: ActiveFileEventType,
    event_data: ActiveFileEventData
}

////////////////////////////////////////////////////
///  FileEvent 初始化方法
///////////////////////////////////////////////////
impl ActiveFileEvent {
    pub fn new(
        event_info: EventInfo,
        event_type: ActiveFileEventType,
        event_data: ActiveFileEventData,
    ) -> Self {
        Self {
            event_info,
            event_type,
            event_data,
        }
    }
}

////////////////////////////////////////////////////
///  FileEvent 的属性方法
///////////////////////////////////////////////////
impl ActiveFileEvent {
    pub fn event_info(&self) -> EventInfo {
        self.event_info.clone()
    }
    pub fn event_type(&self) -> ActiveFileEventType {
        self.event_type.clone()
    }
    pub fn event_data(&self) -> ActiveFileEventData {
        self.event_data.clone()
    }
}