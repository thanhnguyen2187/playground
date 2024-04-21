function addShakeTracking(executeFn: () => void) {
  if (typeof window.DeviceMotionEvent != 'undefined') {
    // Shake sensitivity (a lower number is more)
    const sensitivity = 16;

    // Position variables
    let x1 = 0, y1 = 0, z1 = 0, x2 = 0, y2 = 0, z2 = 0;

    // Listen to motion events and update the position
    window.addEventListener('devicemotion', function (e) {
      x1 = e.accelerationIncludingGravity?.x ?? 0;
      y1 = e.accelerationIncludingGravity?.y ?? 0;
      z1 = e.accelerationIncludingGravity?.z ?? 0;
    }, false);

    // Periodically check the position and fire
    // if the change is greater than the sensitivity
    setInterval(function () {
      const change = Math.abs(x1-x2+y1-y2+z1-z2);

      if (change > sensitivity) {
        executeFn()
      }

      // Update new position
      x2 = x1;
      y2 = y1;
      z2 = z1;
    }, 200);
  }
}