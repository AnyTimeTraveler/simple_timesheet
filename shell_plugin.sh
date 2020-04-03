function work() {
  if test -f ~/working; then
    echo -n "Leave: " | tee -a ~/times
    rm ~/working
  else
    echo -n "Enter: " | tee -a ~/times
    touch ~/working
  fi
  date --rfc-3339=seconds | tee -a ~/times
}

alias w="work"

function work_status() {
  if test -f ~/working; then
    echo '(w)'
  fi
}

alias wt="~/bin/simple_timesheet ~/times"
