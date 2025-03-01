use std::collections::HashMap;

pub enum LiveDSLASTNode {
  Import(ImportNode),
  Constant(ConstantNode),
  Widget(WidgetNode),
}

#[derive(Debug, Clone)]
pub struct ImportNode {}


#[derive(Debug, Clone)]
pub struct ConstantNode {
  pub name: String,
  pub value: String,
}

#[derive(Debug, Clone)]
pub struct WidgetNode {
  pub name: String,
  pub widget_type: String,
  pub properties: HashMap<String, Expression>,
  pub children: Vec<WidgetNode>,
}

#[derive(Debug, Clone)]
pub enum Expression {
  Color(String),
  Number(f64),
  Boolean(bool),
  String(String),
}




#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parsed_token() {
    let raw_source_code: &str = r#"
      live_design! {
          use link::theme::*;
          use link::shaders::*;
          use link::widgets::*;

          use crate::shared::styles::*;
          use crate::home::home_screen::HomeScreen;
          use crate::profile::my_profile_screen::MyProfileScreen;
          use crate::verification_modal::VerificationModal;
          use crate::login::login_screen::LoginScreen;
          use crate::shared::popup_list::PopupList;

          ICON_CHAT = dep("crate://self/resources/icons/chat.svg")
          ICON_CONTACTS = dep("crate://self/resources/icons/contacts.svg")
          ICON_DISCOVER = dep("crate://self/resources/icons/discover.svg")
          ICON_ME = dep("crate://self/resources/icons/me.svg")


          APP_TAB_COLOR = #344054
          APP_TAB_COLOR_HOVER = #636e82
          APP_TAB_COLOR_SELECTED = #091

          AppTab = <RadioButton> {
              width: Fit,
              height: Fill,
              flow: Down,
              align: {x: 0.5, y: 0.5},

              icon_walk: {width: 20, height: 20, margin: 0.0}
              label_walk: {margin: 0.0}

              draw_radio: {
                  radio_type: Tab,

                  // Draws a horizontal line under the tab when selected or hovered.
                  fn pixel(self) -> vec4 {
                      let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                      sdf.box(
                          20.0,
                          self.rect_size.y - 2.5,
                          self.rect_size.x - 40,
                          self.rect_size.y - 4,
                          0.5
                      );
                      sdf.fill(
                          mix(
                              mix(
                                  #0000,
                                  (APP_TAB_COLOR_HOVER),
                                  self.hover
                              ),
                              (APP_TAB_COLOR_SELECTED),
                              self.selected
                          )
                      );
                      return sdf.result;
                  }
              }

              draw_text: {
                  color_unselected: (APP_TAB_COLOR)
                  color_unselected_hover: (APP_TAB_COLOR_HOVER)
                  color_selected: (APP_TAB_COLOR_SELECTED)

                  fn get_color(self) -> vec4 {
                      return mix(
                          mix(
                              self.color_unselected,
                              self.color_unselected_hover,
                              self.hover
                          ),
                          self.color_selected,
                          self.selected
                      )
                  }
              }

              draw_icon: {
                  instance color_unselected: (APP_TAB_COLOR)
                  instance color_unselected_hover: (APP_TAB_COLOR_HOVER)
                  instance color_selected: (APP_TAB_COLOR_SELECTED)
                  fn get_color(self) -> vec4 {
                      return mix(
                          mix(
                              self.color_unselected,
                              self.color_unselected_hover,
                              self.hover
                          ),
                          self.color_selected,
                          self.selected
                      )
                  }
              }
          }

          App = {{App}} {
              ui: <Window> {
                  window: {inner_size: vec2(1280, 800)},
                  pass: {clear_color: #2A}

                  body = {
                      // A wrapper view for showing top-level app modals/dialogs/popups
                      <View> {
                          width: Fill, height: Fill,
                          flow: Overlay,

                          home_screen_view = <View> {
                              visible: false
                              home_screen = <HomeScreen> {}
                          }
                          login_screen_view = <View> {
                              visible: true
                              login_screen = <LoginScreen> {}
                          }
                          popup = <PopupNotification> {
                              margin: {top: 45, right: 13},
                              content: {
                                  <PopupList> {}
                              }
                          }
                          verification_modal = <Modal> {
                              content: {
                                  verification_modal_inner = <VerificationModal> {}
                              }
                          }

                          // message_source_modal = <Modal> {
                          //     content: {
                          //         message_source_modal_inner = <MessageSourceModal> {}
                          //     }
                          // }
                      }
                  } // end of body
              }
          }
      }
      "#;

      println!("{:?}", raw_source_code);
  }
}
