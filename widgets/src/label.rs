use {
    crate::{
        makepad_derive_widget::*,
        makepad_draw::*,
        widget::*
    }
};

live_design!{
    import makepad_draw::shader::std::*;
    import makepad_widgets::theme::*;
    Label= {{Label}} {
        
            width:Fit
            height:Fit
        
        draw_label:{
            color:#8,
            wrap: Word
        }
    }
}

#[derive(Live)]
pub struct Label {
    #[live] draw_label: DrawText,
    #[walk] walk: Walk,
    #[live] align: Align,

    //margin: Margin,
    #[live] label: RcStringMut,
} 

impl LiveHook for Label{
    fn before_live_design(cx:&mut Cx){
        register_widget!(cx,Label)
    }
}

impl Widget for Label {
    fn redraw(&mut self, cx:&mut Cx){
        self.draw_label.redraw(cx)
    }
    
    fn walk(&self)->Walk{
        self.walk
    }
    
    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk:Walk)->WidgetDraw{
        self.draw_label.draw_walk(cx, walk, self.align, self.label.as_ref());
        WidgetDraw::done()
    }
}


#[derive(Clone, PartialEq, WidgetRef)]
pub struct LabelRef(WidgetRef); 

impl LabelRef{
    pub fn set_label(&self, text:&str){
        if let Some(mut inner) = self.borrow_mut(){
            let s = inner.label.as_mut();
            s.clear();
            s.push_str(text);
        }
    }
}
