;; --- Base ---

; HTML Tags: <input />
;             ^^^^^
((tag_name) @tag)

; HTML Comments: <!-- ... -->
;                ^^^^^^^^^^^^
((comment) @comment)

; Attributes: <a href="#" />
;                ^^^^  ^
(attribute
    (attribute_name) @attribute
    (quoted_attribute_value) @string)

; HTML Start Tag: <button>...</button>
;                 ^      ^
(start_tag
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

; HTML End Tag: <button>...</button>
;                          ^^      ^
(end_tag
  "</" @punctuation.bracket
  ">" @punctuation.bracket)

; HTML Self-Closing Tag: <input />
;                        ^      ^^
(self_closing_tag
  "<" @punctuation.bracket
  "/>" @punctuation.bracket)



;; --- Bindings ---

; Event Binding: (submit)="onSubmit($event)"
;                ^^^^^^^^
(event_binding
    "(" @punctuation.bracket
    (binding_name (identifier) @attribute)
    ")" @punctuation.bracket)

; Event Binding Keyword: (submit)="onSubmit($event)"
;                                           ^^^^^^
((identifier) @keyword.special (#eq? @keyword.special "$event"))

; Property Binding: [disabled]="isLoading()"
;                   ^^^^^^^^^^
(property_binding
    "[" @punctuation.bracket
    (binding_name (identifier) @attribute)
    "]" @punctuation.bracket)

; Class Binding: <a href="#" [class.active]="..." />
;                            ^^^^^^^^^^^^^
(property_binding
    "[" @punctuation.bracket
    (class_binding) @attribute
    "]" @punctuation.bracket)

; Named Binding: <span [style.color]="themeColor">
;                      ^^^^^^^^^^^^^
(property_binding
    "[" @punctuation.bracket
    (binding_name) @attribute
    "]" @punctuation.bracket)

; Two-way Binding: [(ngModel)]="username"
;                  ^^^^^^^^^^^  ^^^^^^^^
(two_way_binding
    "[(" @punctuation.bracket
    (binding_name (identifier) @attribute)
    ")]" @punctuation.bracket
    (expression (identifier) @variable))



;; --- Control Flows ---

; Keywords: @if (...) { }
;           ^^^
("@" @keyword.special
    (control_keyword) @keyword.special)

; Special Keywords: @for (snack of favoriteSnacks; track snack.id)
;                               ^^                 ^^^^^
((special_keyword) @keyword.special)

; Loop Variable: @for (snack of favoriteSnacks; track snack.id)
;                      ^^^^^
(for_declaration
    (identifier) @variable)

; Loop Variable Aliasing: @for (snack of favoriteSnacks; track snack.id; let idx = $index)
;                                                                                  ^^^^^^
(for_reference
    alias: (assignment_expression
        value: (expression) @variable.special (#match? @variable.special "^\\$(index|first|last|even|odd|count|event)$")))



;; --- Directives ---

; Structural: *ngFor="..."
;             ^^^^^
(structural_directive
    "*" @keyword.special
    (identifier) @string.special)



;; --- Interpolation ---

; Delimiters: Hello {{ user.name }}
;                   ^^           ^^
(interpolation
    "{{" @punctuation.bracket
    "}}" @punctuation.bracket)



;; --- Template ---

; Structural Assignment: <li *ngFor="let item of items; trackBy: trackById">
;                                             ^^
(structural_assignment
    operator: (identifier) @keyword.special)

; Structural Assignment Special Keyword: <li *ngFor="let item of items; trackBy: trackById">
;                                                                       ^^^^^^^
(structural_assignment
    name: (identifier) @keyword.special (#match? @keyword.special "^(trackBy)$"))

; Template Variable: <input #username />
;                           ^^^^^^^^^
(attribute
  (attribute_name) @variable.special (#match? @variable.special "^#"))
