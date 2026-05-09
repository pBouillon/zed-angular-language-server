;; --- Bindings ---

; Event Binding Logic: (click)="moveToPage(pageOffset + 1)"
;                               ^^^^^^^^^^^^^^^^^^^^^^^^^^
(event_binding
    [
        (expression)
        (binary_expression)
        (ternary_expression)
        (conditional_expression)
        (concatenation_expression)
    ] @content (#set! injection.language "typescript"))

; Class Binding Expression: [class.active]="isActive && other"
;                                           ^^^^^^^^^^^^^^^^^
(property_binding
  (class_binding)
  [
      (expression)
      (binary_expression)
      (ternary_expression)
      (conditional_expression)
      (concatenation_expression)
  ] @content (#set! injection.language "typescript"))

; Style Binding Expression: [style.color]="isDark ? '#000' : '#fff'"
;                                          ^
(property_binding
    (binding_name)
    [
        (expression)
        (binary_expression)
        (ternary_expression)
        (conditional_expression)
        (concatenation_expression)
    ] @content (#set! injection.language "typescript"))



;; --- Control Flow ---

; Condition: @if (a > b) { } @else if (b > a) { }
;                 ^^^^^                ^^^^^
((if_condition) @content (#set! injection.language "typescript"))

; Let declarations: @let name = 'Alice';
;                               ^^^^^^^
(let_statement
    (assignment_expression) @content (#set! injection.language "typescript"))

; For loop expressions: @for (snack of favoriteSnacks; track snack.id)
;                                      ^^^^^^^^^^^^^^        ^^^^^^^^
(for_declaration
    (expression) @content (#set! injection.language "typescript"))

; Switch Expression: @switch (user.status) { }
;                             ^^^^^^^^^^^
(switch_statement
    value: (expression) @content (#set! injection.language "typescript"))

; Case Statement: @case ('active') { }
;                        ^^^^^^^^
(case_statement
    value: (expression) @content (#set! injection.language "typescript"))



;; --- Directive ---

; Structural Directive Expression: <span *ngSwitchCase="'active'">Active</span>
;                                                       ^^^^^^^^
(structural_directive
    (structural_expression) @content (#set! injection.language "typescript"))



;; --- Interpolation ---

; Content: {{ 'app.hello' | translate: { name: 'world' } }}
;             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
(interpolation
    [
        (expression)
        (binary_expression)
        (ternary_expression)
        (conditional_expression)
        (concatenation_expression)
    ] @content (#set! injection.language "typescript"))



;; --- Editor Configuration ---

; Allows variables defined in one expression to be seen by other expressions in the same file.
(#set! injection.combined)
