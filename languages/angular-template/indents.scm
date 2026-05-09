;; --- HTML Structure ---

; Opening tag: <div>
;                  ^
(start_tag ">" @end) @indent

; Self-closing tag: <img />
;                        ^^
(self_closing_tag "/>" @end) @indent

; Element block: <div>...</div>
;                ^^^        ^^^
(element
    (start_tag) @start
    (end_tag)? @end) @indent



;; --- Control Flow ---

; All block bodies: @if { }
;                       ^ ^
(statement_block "{" "}" @end) @indent

; Switch body: @switch (user.status) { ... }
;                                   ^      ^
(switch_body "{" "}" @end) @indent
