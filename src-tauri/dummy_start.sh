#!/bin/bash
python -c "import time, sys; print('child running'); sys.stdout.flush(); time.sleep(100)" &
wait
