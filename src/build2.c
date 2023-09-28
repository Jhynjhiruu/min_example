int build1_function(void);

int build2_function(void) {
    return build1_function() + 1;
}