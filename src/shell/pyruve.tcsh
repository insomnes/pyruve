alias _pyruve_apply 'set _pyruve_activation_script = "`pyruve hook tcsh`"; set _pyruve_status = $status; if ($_pyruve_status == 0) source "$_pyruve_activation_script"; if ($_pyruve_status == 10) eval deactivate; unset _pyruve_activation_script _pyruve_status';
set _pyruve_existing_cwdcmd = "`alias cwdcmd`";
if ( "$_pyruve_existing_cwdcmd" == "" ) alias cwdcmd '_pyruve_apply';
if ( "$_pyruve_existing_cwdcmd" != "" && "$_pyruve_existing_cwdcmd" !~ *"_pyruve_apply"* ) alias _pyruve_previous_cwdcmd "$_pyruve_existing_cwdcmd";
if ( "$_pyruve_existing_cwdcmd" != "" && "$_pyruve_existing_cwdcmd" !~ *"_pyruve_apply"* ) alias cwdcmd '_pyruve_previous_cwdcmd; _pyruve_apply';
unset _pyruve_existing_cwdcmd;
