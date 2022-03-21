import { invoke } from '@tauri-apps/api';
import { getCurrent } from '@tauri-apps/api/window';

export class ScreenLock {

  constructor () {
    this.locked = false;
  }

  Start(seconds) {
    let counter = seconds;
    //console.log(this.winmanager);
    let winmanager = getCurrent();

    invoke("set_screen_lock", {val: true});
    winmanager.setAlwaysOnTop(true);
    winmanager.setFocus();
    winmanager.setFullscreen(true);
    this.locked = true;
    
    const interval = setInterval(() => {
      
      counter--;
        
      if (counter < 0 ) {
        clearInterval(interval);
        winmanager.setAlwaysOnTop(false);
        winmanager.setFullscreen(false);
        invoke("set_screen_lock", {val: false});
        this.locked = false;
      }
    }, 1000);
  } 

  Block() {
    let winmanager = getCurrent();
    //console.log(winmanager);
    if(this.locked) {
          winmanager.setFullscreen(false);
          winmanager.setAlwaysOnTop(false);    
          
          invoke("set_screen_lock", {val: false});
          this.locked = false;
    } else {
      invoke("set_screen_lock", {val: true});
      winmanager.setFocus();
      winmanager.setAlwaysOnTop(true);
      winmanager.setFullscreen(true);
      
      
      
      this.locked = true;
    }
  }
    
}
