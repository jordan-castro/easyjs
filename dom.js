function get_element_by_id(id) { document.getElementById(id); }
function get_elements_by_class_name(class_name) { document.getElementsByClassName(class_name); }
function get_elements_by_tag(tag) { return document.getElementsByTagName(tag); }
function get_element_by_tag(tag) { return get_elements_by_tag(tag)[0]; }