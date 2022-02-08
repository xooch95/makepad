use {
    std::collections::BTreeMap,
    crate::{
        makepad_platform::*,
        makepad_platform::audio::*,
        audio_graph::*
        //audio_engine::AudioEngine
    }
};


// Audio component API



pub enum AudioComponentAction {}

pub trait AudioComponent: LiveApply {
    fn handle_event_with_fn(&mut self, _cx: &mut Cx, event: &mut Event, _dispatch_action: &mut dyn FnMut(&mut Cx, AudioComponentAction));
    fn type_id(&self) -> LiveType;
    fn get_graph_node(&mut self) -> Box<dyn AudioGraphNode + Send>;
}

pub trait AudioGraphNode {
    fn handle_midi_1_data(&mut self, data: Midi1Data);
    fn render_to_audio_buffer(&mut self, buffer: &mut AudioBuffer);
}

generate_ref_cast_api!(AudioComponent);



// Audio component registry


#[derive(Default)]
pub struct AudioComponentRegistry {
    pub map: BTreeMap<LiveType, (LiveComponentInfo, Box<dyn AudioComponentFactory>)>
}

impl AudioComponentRegistry {
    fn new(&self, cx: &mut Cx, ty: LiveType) -> Option<Box<dyn AudioComponent >> {
        self.map.get(&ty).map( | (_, fac) | fac.new(cx))
    }
}

pub trait AudioComponentFactory {
    fn new(&self, cx: &mut Cx) -> Box<dyn AudioComponent>;
}

impl LiveComponentRegistry for AudioComponentRegistry {
    fn type_id(&self) -> LiveType {LiveType::of::<AudioComponentRegistry>()}
    fn component_type(&self) -> LiveId {id!(AudioComponent)}
    fn get_component_infos(&self) -> Vec<LiveComponentInfo> {
        self.map.values().map( | (info, _) | info.clone()).collect()
    }
    fn get_component_info(&self, name:LiveId)->Option<LiveComponentInfo>{
        self.map.values().find( | (info, _) | info.name == name).map( | (info, _) | info.clone())
    }
}



// Live bindings for AudioComponentOption


pub struct AudioComponentOption(Option<Box<dyn AudioComponent >>);

impl AudioComponentOption {
    pub fn component(&mut self) -> &mut Option<Box<dyn AudioComponent >> {
        &mut self.0
    }
}

impl LiveHook for AudioComponentOption {}
impl LiveApply for AudioComponentOption {
    fn apply(&mut self, cx: &mut Cx, apply_from: ApplyFrom, index: usize, nodes: &[LiveNode]) -> usize {
        if let LiveValue::Class {live_type, ..} = nodes[index].value {
            if let Some(component) = &mut self.0 {
                if component.type_id() != live_type {
                    self.0 = None;
                }
                else {
                    component.apply(cx, apply_from, index, nodes);
                    return nodes.skip_node(index);
                }
            }
            if let Some(mut component) = cx.live_registry.clone().borrow().components.clone().get::<AudioComponentRegistry>().new(cx, live_type) {
                component.apply(cx, apply_from, index, nodes);
                self.0 = Some(component);
            }
        }
        else {
            if let Some(component) = &mut self.0 {
                component.apply(cx, apply_from, index, nodes);
            }
        }
        nodes.skip_node(index)
    }
}

impl LiveNew for AudioComponentOption {
    fn new(_cx: &mut Cx) -> Self {
        Self (None)
    }
    fn new_apply(cx: &mut Cx, apply_from: ApplyFrom, index: usize, nodes: &[LiveNode]) -> Self {
        let mut ret = Self (None);
        ret.apply(cx, apply_from, index, nodes);
        ret
    }
    fn live_type_info(_cx: &mut Cx) -> LiveTypeInfo {
        LiveTypeInfo {
            module_id: LiveModuleId::from_str(&module_path!()).unwrap(),
            live_type: LiveType::of::<dyn AudioComponent>(),
            fields: Vec::new(),
            type_name: LiveId(0)
        }
    }
}


#[macro_export]
macro_rules!register_as_audio_component {
    ( $ cx: expr, $ ty: ident) => {
        {
            struct Factory();
            impl AudioComponentFactory for Factory {
                fn new(&self, cx: &mut Cx) -> Box<dyn AudioComponent> {
                    Box::new( $ ty::new(cx))
                }
            }
            $ cx.live_registry.borrow().components.clone().get_or_create::<AudioComponentRegistry>()
                .map.insert(
                LiveType::of::< $ ty>(),
                (
                    LiveComponentInfo {
                        name: LiveId::from_str(stringify!( $ ty)).unwrap(),
                        module_id: LiveModuleId::from_str(&module_path!()).unwrap()
                    },
                    Box::new(Factory())
                )
            );
        }
    }
}