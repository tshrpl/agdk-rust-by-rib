
# issues
when i try to resize the window in windows build, it becomes extremely laggy
and when i try to change the device orientation in android build, it stops responding

# adb logcat output (filtered)
```console
05-21 22:41:48.832  2371  2371 D Launcher.CellLayout: touch item:ShortcutInfo, id=153, itemType=0, user=UserHandle{0}, mIconType=0, pkgName=co.realfit.agdkwinitwgpu, className=co.realfit.agdkwini
twgpu.MainActivity, screenId=4, container=-100, cellX=0, cellY=1, spanX=1, spanY=1, isLandscapePos=false
05-21 22:41:48.874  1602  2058 I ActivityTaskManager: START u0 {act=android.intent.action.MAIN cat=[android.intent.category.LAUNCHER] flg=0x10200000 cmp=co.realfit.agdkwinitwgpu/.MainActivity bnds=[40,323][144,427] (has extras)} from uid 10086
05-21 22:41:48.913  1602  1652 W ActivityManager: ProcessRecord{6d514bd 0:co.realfit.agdkwinitwgpu/u0a295}: ART verification disabled
05-21 22:41:48.942   527   527 D OemNetd : setPidForPackage: packageName=co.realfit.agdkwinitwgpu, pid=9054, pid=10295
05-21 22:41:48.942  1602  1671 I ActivityManager: Start proc 9054:co.realfit.agdkwinitwgpu/u0a295 for activity {co.realfit.agdkwinitwgpu/co.realfit.agdkwinitwgpu.MainActivity} caller=com.miui.home
05-21 22:41:49.034  5050 12895 D PerfEngineController: ForegroundInfo{mForegroundPackageName='co.realfit.agdkwinitwgpu', mForegroundUid=10295, mForegroundPid=9054, mLastForegroundPackageName='com.miui.home', mLastForegroundUid=10086, mLastForegroundPid=2371, mMultiWindowForegroundPackageName='null', mMultiWindowForegroundUid=-1, mFlags=1}
05-21 22:41:49.034  6492  6508 D AutoInstallService: onForegroundInfoChanged: co.realfit.agdkwinitwgpu
05-21 22:41:49.037  4112 11745 I ProcessMonitor: onForegroundInfoChanged: ForegroundInfo{mForegroundPackageName='co.realfit.agdkwinitwgpu', mForegroundUid=10295, mForegroundPid=9054, mLastForegroundPackageName='com.miui.home', mLastForegroundUid=10086, mLastForegroundPid=2371, mMultiWindowForegroundPackageName='null', mMultiWindowForegroundUid=-1, mFlags=1}
05-21 22:41:49.037  5073  5097 I GST     : forePkg: co.realfit.agdkwinitwgpu, preForePkg: com.miui.home
05-21 22:41:49.038  4112 11745 I GameBoosterService: onForegroundInfoChanged: Cur=co.realfit.agdkwinitwgpu       last=com.miui.home
05-21 22:41:49.039  4112 11745 D GameBoosterService: onGameStatusChange foreground:ForegroundInfo{mForegroundPackageName='co.realfit.agdkwinitwgpu', mForegroundUid=10295, mForegroundPid=9054, mLastForegroundPackageName='com.miui.home', mLastForegroundUid=10086, mLastForegroundPid=2371, mMultiWindowForegroundPackageName='null', mMultiWindowForegroundUid=-1, mFlags=1}
05-21 22:41:49.071  6365  6365 D TouchAssistant: getPackageNameFromPid packageName:co.realfit.agdkwinitwgpu
05-21 22:41:49.071  6365  6365 D TouchAssistant: onTopAppChanged packageName:co.realfit.agdkwinitwgpu
05-21 22:41:49.252  9054  9054 D ForceDarkHelper: updateByCheckExcludeList: pkg: co.realfit.agdkwinitwgpu activity: co.realfit.agdkwinitwgpu.MainActivity@caad8f4
05-21 22:41:49.257  9054  9054 D ForceDarkHelper: updateByCheckExcludeList: pkg: co.realfit.agdkwinitwgpu activity: co.realfit.agdkwinitwgpu.MainActivity@caad8f4
05-21 22:41:49.265  9054  9054 D ForceDarkHelper: updateByCheckExcludeList: pkg: co.realfit.agdkwinitwgpu activity: co.realfit.agdkwinitwgpu.MainActivity@caad8f4
05-21 22:41:49.271  9054  9054 D ForceDarkHelper: updateByCheckExcludeList: pkg: co.realfit.agdkwinitwgpu activity: co.realfit.agdkwinitwgpu.MainActivity@caad8f4
05-21 22:41:49.386  9054  9079 D vulkan  : searching for layers in '/data/app/co.realfit.agdkwinitwgpu-n23kXK-XeidIfQZ39ySTVg==/lib/arm64'
05-21 22:41:49.387  9054  9079 D vulkan  : searching for layers in '/data/app/co.realfit.agdkwinitwgpu-n23kXK-XeidIfQZ39ySTVg==/base.apk!/lib/arm64-v8a'
05-21 22:41:49.472  9054  9079 D vulkan  : added global layer 'VK_LAYER_KHRONOS_validation' from library '/data/app/co.realfit.agdkwinitwgpu-n23kXK-XeidIfQZ39ySTVg==/base.apk!/lib/arm64-v8a/libVkLayer_khronos_validation.so'
05-21 22:41:49.752  9054  9079 I chatty  : uid=10295(co.realfit.agdkwinitwgpu) identical 22 lines
05-21 22:41:49.753  9054  9079 I chatty  : uid=10295(co.realfit.agdkwinitwgpu) identical 10 lines
05-21 22:41:49.753  9054  9079 I chatty  : uid=10295(co.realfit.agdkwinitwgpu) identical 2 lines
05-21 22:41:49.754  9054  9079 I chatty  : uid=10295(co.realfit.agdkwinitwgpu) identical 2 lines
05-21 22:41:49.754  9054  9079 I chatty  : uid=10295(co.realfit.agdkwinitwgpu) identical 10 lines
05-21 22:41:49.757  9054  9054 W Activity: Slow Operation: Activity co.realfit.agdkwinitwgpu/.MainActivity onStart took 356ms
05-21 22:41:49.791  9054  9054 W Looper  : Slow Looper main: Activity co.realfit.agdkwinitwgpu/.MainActivity is 708ms late (wall=0ms running=0ms ClientTransaction{ callbacks=[android.app.servertransaction.TopResumedActivityChangeItem] }) because of 2 msg, msg 1 took 108ms (seq=2 late=8ms h=android.app.ActivityThread$H w=110), msg 2 took 627ms (seq=3 late=82ms h=android.app.ActivityThread$H w=159)
05-21 22:41:49.883  1602  1653 I Timeline: Timeline: Activity_windows_visible id: ActivityRecord{3e7ed02 u0 co.realfit.agdkwinitwgpu/.MainActivity t3856} time:128973587
05-21 22:41:49.886  1602  1668 I ActivityTaskManager: Displayed co.realfit.agdkwinitwgpu/.MainActivity: +1s4ms
05-21 22:41:49.937  3560  3560 I GoogleInputMethodService: GoogleInputMethodService.onStartInput():1885 onStartInput(EditorInfo{inputType=0x0(NULL) imeOptions=0x0 privateImeOptions=null actionName=UNSPECIFIED actionLabel=null actionId=0 initialSelStart=-1 initialSelEnd=-1 initialCapsMode=0x0 hintText=null label=null packageName=co.realfit.agdkwinitwgpu fieldId=-1 fieldName=null extras=null}, false)
05-21 22:41:49.987  6365  6365 D TouchAssistant: topActivity.getPackageName(),packageName:co.realfit.agdkwinitwgpu
05-21 22:41:49.987  6365  6365 D TouchAssistant: onTopAppChanged packageName:co.realfit.agdkwinitwgpu
05-21 22:41:52.912  3560  3560 I GoogleInputMethodService: GoogleInputMethodService.onStartInput():1885 onStartInput(EditorInfo{inputType=0x0(NULL) imeOptions=0x0 privateImeOptions=null actionName=UNSPECIFIED actionLabel=null actionId=0 initialSelStart=-1 initialSelEnd=-1 initialCapsMode=0x0 hintText=null label=null packageName=co.realfit.agdkwinitwgpu fieldId=-1 fieldName=null extras=null}, true)
05-21 22:41:54.693  1602  1652 W WindowManager: Force clearing orientation change: Window{623746b u0 co.realfit.agdkwinitwgpu/co.realfit.agdkwinitwgpu.MainActivity}
05-21 22:41:54.843  1602  1652 W WindowManager: Force clearing freeze: AppWindowToken{1d38050 token=Token{582b613 ActivityRecord{3e7ed02 u0 co.realfit.agdkwinitwgpu/.MainActivity t3856}}}
05-21 22:41:54.847  1602  1652 I WindowManager: Screen frozen for +2s156ms due to Window{623746b u0 co.realfit.agdkwinitwgpu/co.realfit.agdkwinitwgpu.MainActivity}
05-21 22:42:05.765  1602  2000 I InputDispatcher: Application is not responding: Window{623746b u0 co.realfit.agdkwinitwgpu/co.realfit.agdkwinitwgpu.MainActivity}.  It has been 8005.6ms since event, 8003.8ms since wait started.  Reason: Waiting to send non-key event because the touched window has not finished processing certain input events that were delivered to it over 500.0ms ago.  Wait queue length: 11.  Wait queue head age: 8530.6ms.
05-21 22:42:05.766  1602  2000 I WindowManager: Input event dispatching timed out sending to co.realfit.agdkwinitwgpu/co.realfit.agdkwinitwgpu.MainActivity.  Reason: Waiting to send non-key event because the touched window has not finished processing certain input events that were delivered to it over 500.0ms ago.  Wait queue length: 11.  Wait queue head age: 8530.6ms.
05-21 22:42:08.997  1602  9967 W ActivityManager: Missing app error report, app = co.realfit.agdkwinitwgpu crashing = false notResponding = true
05-21 22:42:09.536  1602  9415 W ActivityManager: Missing app error report, app = co.realfit.agdkwinitwgpu crashing = false notResponding = true
05-21 22:42:10.075  1602  9415 W ActivityManager: Missing app error report, app = co.realfit.agdkwinitwgpu crashing = false notResponding = true
05-21 22:42:10.616  1602  8899 W ActivityManager: Missing app error report, app = co.realfit.agdkwinitwgpu crashing = false notResponding = true
05-21 22:42:11.156  1602  8899 W ActivityManager: Missing app error report, app = co.realfit.agdkwinitwgpu crashing = false notResponding = true
05-21 22:42:15.154  1602  2000 E ActivityManager: ANR in co.realfit.agdkwinitwgpu (co.realfit.agdkwinitwgpu/.MainActivity)
05-21 22:42:15.154  1602  2000 E ActivityManager: Reason: Input dispatching timed out (co.realfit.agdkwinitwgpu/co.realfit.agdkwinitwgpu.MainActivity, Waiting to send non-key event because the touched window has not finished processing certain input events that were delivered to it over 500.0ms ago.  Wait queue length: 11.  Wait queue head age: 8530.6ms.)
05-21 22:42:15.154  1602  2000 E ActivityManager: Parent: co.realfit.agdkwinitwgpu/.MainActivity
05-21 22:42:15.154  1602  2000 E ActivityManager:   1.6% 9054/co.realfit.agdkwinitwgpu: 1.1% user + 0.4% kernel / faults: 3558 minor
05-21 22:42:16.982  1602  1650 W ActivityTaskManager:   Force finishing activity co.realfit.agdkwinitwgpu/.MainActivity
05-21 22:42:16.988  1602  1650 I ActivityManager: Killing 9054:co.realfit.agdkwinitwgpu/u0a295 (adj 0): user request after error
05-21 22:42:17.014  1602  2055 E InputDispatcher: Window handle Window{1253634 u0 Application Not Responding: co.realfit.agdkwinitwgpu} has no registered input channel
05-21 22:42:17.107  1602  2000 W InputDispatcher: channel '623746b co.realfit.agdkwinitwgpu/co.realfit.agdkwinitwgpu.MainActivity (server)' ~ Consumer closed input channel or an error occurred.events=0xd
05-21 22:42:17.107  1602  2000 E InputDispatcher: channel '623746b co.realfit.agdkwinitwgpu/co.realfit.agdkwinitwgpu.MainActivity (server)' ~ Channel is unrecoverably broken and will be disposed!
05-21 22:42:17.123  1602  9967 I WindowManager: WIN DEATH: Window{623746b u0 co.realfit.agdkwinitwgpu/co.realfit.agdkwinitwgpu.MainActivity}
05-21 22:42:17.123  1602  9967 W InputDispatcher: Attempted to unregister already unregistered input channel '623746b co.realfit.agdkwinitwgpu/co.realfit.agdkwinitwgpu.MainActivity (server)'
05-21 22:42:17.311  5050 12893 D PerfEngineController: ForegroundInfo{mForegroundPackageName='com.miui.home', mForegroundUid=10086, mForegroundPid=2371, mLastForegroundPackageName='co.realfit.agdkwinitwgpu', mLastForegroundUid=10295, mLastForegroundPid=9054, mMultiWindowForegroundPackageName='null', mMultiWindowForegroundUid=-1, mFlags=0}
05-21 22:42:17.312  4112 10742 I ProcessMonitor: onForegroundInfoChanged: ForegroundInfo{mForegroundPackageName='com.miui.home', mForegroundUid=10086, mForegroundPid=2371, mLastForegroundPackageName='co.realfit.agdkwinitwgpu', mLastForegroundUid=10295, mLastForegroundPid=9054, mMultiWindowForegroundPackageName='null', mMultiWindowForegroundUid=-1, mFlags=0}
05-21 22:42:17.314  5073  5097 I GST     : forePkg: com.miui.home, preForePkg: co.realfit.agdkwinitwgpu
05-21 22:42:17.322  4112 10742 I GameBoosterService: onForegroundInfoChanged: Cur=com.miui.home  last=co.realfit.agdkwinitwgpu
05-21 22:42:17.324  4112 10742 D GameBoosterService: onGameStatusChange foreground:ForegroundInfo{mForegroundPackageName='com.miui.home', mForegroundUid=10086, mForegroundPid=2371, mLastForegroundPackageName='co.realfit.agdkwinitwgpu', mLastForegroundUid=10295, mLastForegroundPid=9054, mMultiWindowForegroundPackageName='null', mMultiWindowForegroundUid=-1, mFlags=0}
05-21 22:42:17.407  3560  3560 I GoogleInputMethodService: GoogleInputMethodService.onStartInput():1885 onStartInput(EditorInfo{inputType=0x0(NULL) imeOptions=0x0 privateImeOptions=null actionName=UNSPECIFIED actionLabel=null actionId=0 initialSelStart=-1 initialSelEnd=-1 initialCapsMode=0x0 hintText=null label=null packageName=co.realfit.agdkwinitwgpu fieldId=-1 fieldName=null extras=null}, true)
05-21 22:42:17.512  1602  1652 W ActivityTaskManager: Activity top resumed state loss timeout for ActivityRecord{3e7ed02 u0 co.realfit.agdkwinitwgpu/.MainActivity t-1 f}
```
