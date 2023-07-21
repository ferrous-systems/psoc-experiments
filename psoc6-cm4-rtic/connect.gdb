# A script to just connect to the CM4 side without flashing anything
target extended-remote :3334

monitor arm semihosting enable
break Reset
continue

