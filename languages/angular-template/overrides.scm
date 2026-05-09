;; --- Strings ---

; Quoted attribute value: <a href="...">
;                                  ^^^
[
    (quoted_attribute_value)
] @string



;; --- Expressions ---

; Binding context: (click)="..."
;                           ^^^
[
    (event_binding)
    (property_binding)
    (two_way_binding)
    (interpolation)
] @expression
