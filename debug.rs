mod login
{
    static BUILDER : Option < gtk :: Builder > = None ; fn get_builder() ->
    gtk :: Builder
    {
        if BUILDER . is_none()
        {
            unsafe
            {
                let builder_ptr = unsafe
                {
                    std :: mem :: transmute :: < & Option < gtk :: Builder >,
                    & mut Option < gtk :: Builder >> (& BUILDER)
                } ; builder_ptr .
                replace(gtk :: Builder ::
                        from_string(& stringify !
                                    ("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!-- Generated with glade 3.36.0 -->\n<interface>\n  <requires lib=\"gtk+\" version=\"3.22\"/>\n  <object class=\"GtkWindow\" id=\"window\">\n    <property name=\"can_focus\">False</property>\n    <property name=\"title\" translatable=\"yes\">login</property>\n    <property name=\"icon_name\">avatar-default-symbolic</property>\n    <child>\n      <object class=\"GtkBox\">\n        <property name=\"visible\">True</property>\n        <property name=\"can_focus\">False</property>\n        <property name=\"orientation\">vertical</property>\n        <child>\n          <object class=\"GtkBox\">\n            <property name=\"visible\">True</property>\n            <property name=\"can_focus\">False</property>\n            <property name=\"margin_start\">30</property>\n            <property name=\"margin_end\">30</property>\n            <property name=\"margin_top\">30</property>\n            <property name=\"margin_bottom\">30</property>\n            <property name=\"orientation\">vertical</property>\n            <child>\n              <object class=\"GtkImage\" id=\"icon_image\">\n                <property name=\"visible\">True</property>\n                <property name=\"can_focus\">False</property>\n                <property name=\"halign\">center</property>\n                <property name=\"valign\">center</property>\n                <property name=\"margin_start\">1</property>\n                <property name=\"margin_end\">1</property>\n                <property name=\"margin_top\">1</property>\n                <property name=\"margin_bottom\">1</property>\n                <property name=\"xalign\">0.10000000149011612</property>\n                <property name=\"yalign\">0.10000000149011612</property>\n                <property name=\"pixbuf\">assets\\images\\kiwi.svg</property>\n                <property name=\"icon_size\">6</property>\n              </object>\n              <packing>\n                <property name=\"expand\">False</property>\n                <property name=\"fill\">False</property>\n                <property name=\"position\">0</property>\n              </packing>\n            </child>\n            <child>\n              <object class=\"GtkEntry\" id=\"email_entry\">\n                <property name=\"visible\">True</property>\n                <property name=\"can_focus\">True</property>\n                <property name=\"margin_start\">1</property>\n                <property name=\"margin_end\">1</property>\n                <property name=\"margin_top\">1</property>\n                <property name=\"margin_bottom\">1</property>\n                <property name=\"primary_icon_name\">avatar-default-symbolic</property>\n                <property name=\"placeholder_text\" translatable=\"yes\">email</property>\n                <property name=\"input_purpose\">email</property>\n              </object>\n              <packing>\n                <property name=\"expand\">False</property>\n                <property name=\"fill\">True</property>\n                <property name=\"position\">1</property>\n              </packing>\n            </child>\n            <child>\n              <object class=\"GtkEntry\" id=\"password_entry\">\n                <property name=\"visible\">True</property>\n                <property name=\"can_focus\">True</property>\n                <property name=\"margin_start\">1</property>\n                <property name=\"margin_end\">1</property>\n                <property name=\"margin_top\">1</property>\n                <property name=\"margin_bottom\">1</property>\n                <property name=\"primary_icon_name\">dialog-password-symbolic</property>\n                <property name=\"placeholder_text\" translatable=\"yes\">password</property>\n                <property name=\"input_purpose\">password</property>\n              </object>\n              <packing>\n                <property name=\"expand\">False</property>\n                <property name=\"fill\">True</property>\n                <property name=\"position\">2</property>\n              </packing>\n            </child>\n            <child>\n              <object class=\"GtkBox\">\n                <property name=\"visible\">True</property>\n                <property name=\"can_focus\">False</property>\n                <property name=\"halign\">start</property>\n                <property name=\"valign\">start</property>\n                <child>\n                  <object class=\"GtkCheckButton\" id=\"keep_login_check_button\">\n                    <property name=\"label\" translatable=\"yes\">keep login</property>\n                    <property name=\"visible\">True</property>\n                    <property name=\"can_focus\">True</property>\n                    <property name=\"receives_default\">False</property>\n                    <property name=\"margin_start\">1</property>\n                    <property name=\"margin_end\">1</property>\n                    <property name=\"margin_top\">1</property>\n                    <property name=\"margin_bottom\">1</property>\n                    <property name=\"draw_indicator\">True</property>\n                  </object>\n                  <packing>\n                    <property name=\"expand\">False</property>\n                    <property name=\"fill\">True</property>\n                    <property name=\"position\">0</property>\n                  </packing>\n                </child>\n                <child>\n                  <object class=\"GtkCheckButton\" id=\"force_check_button\">\n                    <property name=\"label\" translatable=\"yes\">force</property>\n                    <property name=\"visible\">True</property>\n                    <property name=\"can_focus\">True</property>\n                    <property name=\"receives_default\">False</property>\n                    <property name=\"draw_indicator\">True</property>\n                  </object>\n                  <packing>\n                    <property name=\"expand\">False</property>\n                    <property name=\"fill\">True</property>\n                    <property name=\"position\">1</property>\n                  </packing>\n                </child>\n              </object>\n              <packing>\n                <property name=\"expand\">False</property>\n                <property name=\"fill\">True</property>\n                <property name=\"position\">3</property>\n              </packing>\n            </child>\n          </object>\n          <packing>\n            <property name=\"expand\">True</property>\n            <property name=\"fill\">True</property>\n            <property name=\"position\">0</property>\n          </packing>\n        </child>\n        <child>\n          <object class=\"GtkButtonBox\">\n            <property name=\"can_focus\">False</property>\n            <property name=\"layout_style\">end</property>\n            <child>\n              <object class=\"GtkButton\" id=\"apply_button\">\n                <property name=\"label\">gtk-apply</property>\n                <property name=\"visible\">True</property>\n                <property name=\"can_focus\">True</property>\n                <property name=\"receives_default\">True</property>\n                <property name=\"margin_start\">1</property>\n                <property name=\"margin_end\">1</property>\n                <property name=\"margin_top\">1</property>\n                <property name=\"margin_bottom\">1</property>\n                <property name=\"use_stock\">True</property>\n                <property name=\"always_show_image\">True</property>\n              </object>\n              <packing>\n                <property name=\"expand\">True</property>\n                <property name=\"fill\">True</property>\n                <property name=\"position\">0</property>\n              </packing>\n            </child>\n            <child>\n              <object class=\"GtkButton\" id=\"cancel_button\">\n                <property name=\"label\">gtk-cancel</property>\n                <property name=\"visible\">True</property>\n                <property name=\"can_focus\">True</property>\n                <property name=\"receives_default\">True</property>\n                <property name=\"margin_start\">1</property>\n                <property name=\"margin_end\">1</property>\n                <property name=\"margin_top\">1</property>\n                <property name=\"margin_bottom\">1</property>\n                <property name=\"use_stock\">True</property>\n                <property name=\"always_show_image\">True</property>\n              </object>\n              <packing>\n                <property name=\"expand\">True</property>\n                <property name=\"fill\">True</property>\n                <property name=\"position\">1</property>\n              </packing>\n            </child>\n          </object>\n          <packing>\n            <property name=\"expand\">False</property>\n            <property name=\"fill\">False</property>\n            <property name=\"position\">1</property>\n          </packing>\n        </child>\n      </object>\n    </child>\n    <child type=\"titlebar\">\n      <placeholder/>\n    </child>\n  </object>\n  <object class=\"GtkMessageDialog\" id=\"id_not_found_message_dialog\">\n    <property name=\"can_focus\">False</property>\n    <property name=\"modal\">True</property>\n    <property name=\"icon_name\">dialog-error-symbolic</property>\n    <property name=\"type_hint\">dialog</property>\n    <property name=\"attached_to\">window</property>\n    <property name=\"message_type\">error</property>\n    <property name=\"text\" translatable=\"yes\">ID not found</property>\n    <property name=\"secondary_text\" translatable=\"yes\">check your email and try again</property>\n    <child internal-child=\"vbox\">\n      <object class=\"GtkBox\">\n        <property name=\"can_focus\">False</property>\n        <property name=\"orientation\">vertical</property>\n        <property name=\"spacing\">2</property>\n        <child internal-child=\"action_area\">\n          <object class=\"GtkButtonBox\">\n            <property name=\"can_focus\">False</property>\n            <property name=\"margin_start\">1</property>\n            <property name=\"margin_end\">1</property>\n            <property name=\"margin_top\">1</property>\n            <property name=\"margin_bottom\">1</property>\n            <property name=\"homogeneous\">True</property>\n            <property name=\"layout_style\">end</property>\n          </object>\n          <packing>\n            <property name=\"expand\">False</property>\n            <property name=\"fill\">False</property>\n            <property name=\"position\">0</property>\n          </packing>\n        </child>\n      </object>\n    </child>\n    <child type=\"titlebar\">\n      <placeholder/>\n    </child>\n  </object>\n  <object class=\"GtkDialog\" id=\"register_device_dialog\">\n    <property name=\"can_focus\">False</property>\n    <property name=\"title\" translatable=\"yes\">register device</property>\n    <property name=\"modal\">True</property>\n    <property name=\"icon_name\">channel-secure-symbolic</property>\n    <property name=\"type_hint\">dialog</property>\n    <property name=\"attached_to\">window</property>\n    <child internal-child=\"vbox\">\n      <object class=\"GtkBox\">\n        <property name=\"can_focus\">False</property>\n        <property name=\"orientation\">vertical</property>\n        <property name=\"spacing\">2</property>\n        <child internal-child=\"action_area\">\n          <object class=\"GtkButtonBox\">\n            <property name=\"can_focus\">False</property>\n            <property name=\"layout_style\">end</property>\n            <child>\n              <object class=\"GtkButton\" id=\"register_device_apply_button\">\n                <property name=\"label\">gtk-apply</property>\n                <property name=\"visible\">True</property>\n                <property name=\"can_focus\">True</property>\n                <property name=\"receives_default\">True</property>\n                <property name=\"use_stock\">True</property>\n                <property name=\"always_show_image\">True</property>\n              </object>\n              <packing>\n                <property name=\"expand\">True</property>\n                <property name=\"fill\">True</property>\n                <property name=\"position\">0</property>\n              </packing>\n            </child>\n            <child>\n              <object class=\"GtkButton\" id=\"register_device_cancel_button\">\n                <property name=\"label\">gtk-cancel</property>\n                <property name=\"visible\">True</property>\n                <property name=\"can_focus\">True</property>\n                <property name=\"receives_default\">True</property>\n                <property name=\"use_stock\">True</property>\n                <property name=\"always_show_image\">True</property>\n              </object>\n              <packing>\n                <property name=\"expand\">True</property>\n                <property name=\"fill\">True</property>\n                <property name=\"position\">1</property>\n              </packing>\n            </child>\n          </object>\n          <packing>\n            <property name=\"expand\">False</property>\n            <property name=\"fill\">False</property>\n            <property name=\"position\">0</property>\n          </packing>\n        </child>\n        <child>\n          <object class=\"GtkBox\">\n            <property name=\"visible\">True</property>\n            <property name=\"can_focus\">False</property>\n            <property name=\"halign\">center</property>\n            <property name=\"valign\">center</property>\n            <child>\n              <object class=\"GtkLabel\">\n                <property name=\"visible\">True</property>\n                <property name=\"can_focus\">False</property>\n                <property name=\"margin_start\">5</property>\n                <property name=\"margin_end\">5</property>\n                <property name=\"margin_top\">1</property>\n                <property name=\"margin_bottom\">1</property>\n                <property name=\"label\" translatable=\"yes\">confirmation code</property>\n              </object>\n              <packing>\n                <property name=\"expand\">False</property>\n                <property name=\"fill\">True</property>\n                <property name=\"position\">0</property>\n              </packing>\n            </child>\n            <child>\n              <object class=\"GtkEntry\" id=\"register_device_code_entry\">\n                <property name=\"visible\">True</property>\n                <property name=\"can_focus\">True</property>\n                <property name=\"margin_start\">1</property>\n                <property name=\"margin_end\">1</property>\n                <property name=\"margin_top\">1</property>\n                <property name=\"margin_bottom\">1</property>\n                <property name=\"max_length\">4</property>\n                <property name=\"width_chars\">4</property>\n                <property name=\"max_width_chars\">4</property>\n                <property name=\"input_purpose\">digits</property>\n              </object>\n              <packing>\n                <property name=\"expand\">False</property>\n                <property name=\"fill\">True</property>\n                <property name=\"position\">1</property>\n              </packing>\n            </child>\n            <child>\n              <object class=\"GtkButton\" id=\"register_device_code_send_button\">\n                <property name=\"label\" translatable=\"yes\">send again</property>\n                <property name=\"visible\">True</property>\n                <property name=\"can_focus\">True</property>\n                <property name=\"receives_default\">True</property>\n                <property name=\"margin_start\">1</property>\n                <property name=\"margin_end\">1</property>\n                <property name=\"margin_top\">1</property>\n                <property name=\"margin_bottom\">1</property>\n                <property name=\"always_show_image\">True</property>\n              </object>\n              <packing>\n                <property name=\"expand\">False</property>\n                <property name=\"fill\">True</property>\n                <property name=\"position\">2</property>\n              </packing>\n            </child>\n          </object>\n          <packing>\n            <property name=\"expand\">False</property>\n            <property name=\"fill\">False</property>\n            <property name=\"position\">1</property>\n          </packing>\n        </child>\n      </object>\n    </child>\n    <child type=\"titlebar\">\n      <placeholder/>\n    </child>\n  </object>\n  <object class=\"GtkMessageDialog\" id=\"device_register_success_message_dialog\">\n    <property name=\"can_focus\">False</property>\n    <property name=\"modal\">True</property>\n    <property name=\"icon_name\">gtk-yes</property>\n    <property name=\"type_hint\">dialog</property>\n    <property name=\"attached_to\">register_device_dialog</property>\n    <property name=\"text\" translatable=\"yes\">Device registered successfully!</property>\n    <property name=\"secondary_text\" translatable=\"yes\">login again</property>\n    <child internal-child=\"vbox\">\n      <object class=\"GtkBox\">\n        <property name=\"can_focus\">False</property>\n        <property name=\"orientation\">vertical</property>\n        <property name=\"spacing\">2</property>\n        <child internal-child=\"action_area\">\n          <object class=\"GtkButtonBox\">\n            <property name=\"can_focus\">False</property>\n            <property name=\"margin_left\">1</property>\n            <property name=\"margin_right\">1</property>\n            <property name=\"margin_start\">1</property>\n            <property name=\"margin_end\">1</property>\n            <property name=\"margin_top\">1</property>\n            <property name=\"margin_bottom\">1</property>\n            <property name=\"layout_style\">end</property>\n            <child>\n              <placeholder/>\n            </child>\n            <child>\n              <placeholder/>\n            </child>\n          </object>\n          <packing>\n            <property name=\"expand\">False</property>\n            <property name=\"fill\">False</property>\n            <property name=\"position\">0</property>\n          </packing>\n        </child>\n      </object>\n    </child>\n    <child type=\"titlebar\">\n      <placeholder/>\n    </child>\n  </object>\n  <object class=\"GtkMessageDialog\" id=\"too_many_confirm_request_message_dialog\">\n    <property name=\"can_focus\">False</property>\n    <property name=\"modal\">True</property>\n    <property name=\"icon_name\">dialog-error-symbolic</property>\n    <property name=\"type_hint\">dialog</property>\n    <property name=\"attached_to\">register_device_dialog</property>\n    <property name=\"message_type\">error</property>\n    <property name=\"text\" translatable=\"yes\">Too many confirm request</property>\n    <property name=\"secondary_text\" translatable=\"yes\">try again later</property>\n    <child internal-child=\"vbox\">\n      <object class=\"GtkBox\">\n        <property name=\"can_focus\">False</property>\n        <property name=\"orientation\">vertical</property>\n        <property name=\"spacing\">2</property>\n        <child internal-child=\"action_area\">\n          <object class=\"GtkButtonBox\">\n            <property name=\"can_focus\">False</property>\n            <property name=\"margin_start\">1</property>\n            <property name=\"margin_end\">1</property>\n            <property name=\"margin_top\">1</property>\n            <property name=\"margin_bottom\">1</property>\n            <property name=\"layout_style\">end</property>\n            <child>\n              <placeholder/>\n            </child>\n            <child>\n              <placeholder/>\n            </child>\n          </object>\n          <packing>\n            <property name=\"expand\">False</property>\n            <property name=\"fill\">False</property>\n            <property name=\"position\">0</property>\n          </packing>\n        </child>\n      </object>\n    </child>\n    <child type=\"titlebar\">\n      <placeholder/>\n    </child>\n  </object>\n  <object class=\"GtkMessageDialog\" id=\"wrong_confirm_code_message_dialog\">\n    <property name=\"can_focus\">False</property>\n    <property name=\"modal\">True</property>\n    <property name=\"icon_name\">dialog-error-symbolic</property>\n    <property name=\"type_hint\">dialog</property>\n    <property name=\"attached_to\">register_device_dialog</property>\n    <property name=\"text\" translatable=\"yes\">Wrong confirm code</property>\n    <property name=\"secondary_text\" translatable=\"yes\">check and try again or re-send confirmation code</property>\n    <child internal-child=\"vbox\">\n      <object class=\"GtkBox\">\n        <property name=\"can_focus\">False</property>\n        <property name=\"orientation\">vertical</property>\n        <property name=\"spacing\">2</property>\n        <child internal-child=\"action_area\">\n          <object class=\"GtkButtonBox\">\n            <property name=\"can_focus\">False</property>\n            <property name=\"margin_left\">1</property>\n            <property name=\"margin_right\">1</property>\n            <property name=\"margin_start\">1</property>\n            <property name=\"margin_end\">1</property>\n            <property name=\"margin_top\">1</property>\n            <property name=\"margin_bottom\">1</property>\n            <property name=\"layout_style\">end</property>\n            <child>\n              <placeholder/>\n            </child>\n            <child>\n              <placeholder/>\n            </child>\n          </object>\n          <packing>\n            <property name=\"expand\">False</property>\n            <property name=\"fill\">False</property>\n            <property name=\"position\">0</property>\n          </packing>\n        </child>\n      </object>\n    </child>\n    <child type=\"titlebar\">\n      <placeholder/>\n    </child>\n  </object>\n  <object class=\"GtkMessageDialog\" id=\"wrong_password_message_dialog\">\n    <property name=\"can_focus\">False</property>\n    <property name=\"modal\">True</property>\n    <property name=\"icon_name\">dialog-error-symbolic</property>\n    <property name=\"type_hint\">dialog</property>\n    <property name=\"attached_to\">window</property>\n    <property name=\"message_type\">error</property>\n    <property name=\"text\" translatable=\"yes\">Wrong password</property>\n    <property name=\"secondary_text\" translatable=\"yes\">check your password and try again</property>\n    <child internal-child=\"vbox\">\n      <object class=\"GtkBox\">\n        <property name=\"can_focus\">False</property>\n        <property name=\"orientation\">vertical</property>\n        <property name=\"spacing\">2</property>\n        <child internal-child=\"action_area\">\n          <object class=\"GtkButtonBox\">\n            <property name=\"can_focus\">False</property>\n            <property name=\"margin_start\">1</property>\n            <property name=\"margin_end\">1</property>\n            <property name=\"margin_top\">1</property>\n            <property name=\"margin_bottom\">1</property>\n            <property name=\"layout_style\">end</property>\n          </object>\n          <packing>\n            <property name=\"expand\">False</property>\n            <property name=\"fill\">False</property>\n            <property name=\"position\">0</property>\n          </packing>\n        </child>\n      </object>\n    </child>\n    <child type=\"titlebar\">\n      <placeholder/>\n    </child>\n  </object>\n</interface>\n")))
            }
        } BUILDER . unwrap()
    } mod force_check_button
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk ::
        CheckButton
        {
            super :: get_builder() .
            get_object(& stringify ! ("force_check_button")) . unwrap()
        }
    } mod register_device_code_send_button
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk :: Button
        {
            super :: get_builder() .
            get_object(& stringify ! ("register_device_code_send_button")) .
            unwrap()
        }
    } mod wrong_password_message_dialog
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk ::
        MessageDialog
        {
            super :: get_builder() .
            get_object(& stringify ! ("wrong_password_message_dialog")) .
            unwrap()
        }
    } mod register_device_cancel_button
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk :: Button
        {
            super :: get_builder() .
            get_object(& stringify ! ("register_device_cancel_button")) .
            unwrap()
        }
    } mod register_device_code_entry
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk :: Entry
        {
            super :: get_builder() .
            get_object(& stringify ! ("register_device_code_entry")) .
            unwrap()
        }
    } mod keep_login_check_button
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk ::
        CheckButton
        {
            super :: get_builder() .
            get_object(& stringify ! ("keep_login_check_button")) . unwrap()
        }
    } mod window
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk :: Window
        {
            super :: get_builder() . get_object(& stringify ! ("window")) .
            unwrap()
        }
    } mod email_entry
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk :: Entry
        {
            super :: get_builder() . get_object(& stringify ! ("email_entry"))
            . unwrap()
        }
    } mod id_not_found_message_dialog
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk ::
        MessageDialog
        {
            super :: get_builder() .
            get_object(& stringify ! ("id_not_found_message_dialog")) .
            unwrap()
        }
    } mod register_device_dialog
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk :: Dialog
        {
            super :: get_builder() .
            get_object(& stringify ! ("register_device_dialog")) . unwrap()
        }
    } mod apply_button
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk :: Button
        {
            super :: get_builder() .
            get_object(& stringify ! ("apply_button")) . unwrap()
        }
    } mod cancel_button
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk :: Button
        {
            super :: get_builder() .
            get_object(& stringify ! ("cancel_button")) . unwrap()
        }
    } mod device_register_success_message_dialog
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk ::
        MessageDialog
        {
            super :: get_builder() .
            get_object(& stringify !
                       ("device_register_success_message_dialog")) . unwrap()
        }
    } mod wrong_confirm_code_message_dialog
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk ::
        MessageDialog
        {
            super :: get_builder() .
            get_object(& stringify ! ("wrong_confirm_code_message_dialog")) .
            unwrap()
        }
    } mod password_entry
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk :: Entry
        {
            super :: get_builder() .
            get_object(& stringify ! ("password_entry")) . unwrap()
        }
    } mod register_device_apply_button
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk :: Button
        {
            super :: get_builder() .
            get_object(& stringify ! ("register_device_apply_button")) .
            unwrap()
        }
    } mod too_many_confirm_request_message_dialog
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk ::
        MessageDialog
        {
            super :: get_builder() .
            get_object(& stringify !
                       ("too_many_confirm_request_message_dialog")) . unwrap()
        }
    } mod icon_image
    {
        use gtk :: prelude :: BuilderExtManual ; fn get() -> gtk :: Image
        {
            super :: get_builder() . get_object(& stringify ! ("icon_image"))
            . unwrap()
        }
    }
}.