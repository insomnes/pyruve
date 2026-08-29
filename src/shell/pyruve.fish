function _pyruve_apply
    set -l activation_script (command pyruve hook fish)
    set -l pyruve_status $status

    switch $pyruve_status
        case 0
            source "$activation_script"
        case 10
            if functions -q deactivate
                deactivate
            end
        case 11
            return 0
        case '*'
            return $pyruve_status
    end
end

functions -e _pyruve_on_cd 2>/dev/null
function _pyruve_on_cd --on-variable PWD
    _pyruve_apply
end

if status is-interactive
    _pyruve_apply
end
