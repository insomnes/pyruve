function _pyruve_on_cd --on-variable PWD
    set output (pyruve)
    set modified_output (string replace "bin/activate" "bin/activate.fish" -- $output)
    eval $modified_output
end

if status is-interactive
    _pyruve_on_cd;
end
