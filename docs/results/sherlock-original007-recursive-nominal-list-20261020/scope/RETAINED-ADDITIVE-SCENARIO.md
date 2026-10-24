#### UIREQ-007 / UIREQ-008 scenario: Every arm has distinct binders
- **WHEN** any arm repeats a source-preorder binder, including an unselected or dedicated-only unplanned arm.
- **THEN** body typing SHALL reject it while permitting separate-arm name reuse and outer lexical shadowing. The diagnostic SHALL retain its typed name and arm-body source path. This restricts the raw owner language's never-selectable repeated dedicated arms and supplies neither exhaustiveness nor general reachability analysis.
