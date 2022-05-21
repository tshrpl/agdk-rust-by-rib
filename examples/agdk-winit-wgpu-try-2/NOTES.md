
# issues
resizing the window is not laggy in windows build
but the android build instantly exists

# adb logcat output (filtered)
```console
05-21 23:34:03.687  2371  2371 D Launcher.CellLayout: touch item:ShortcutInfo, id=154, itemType=0, user=UserHandle{0}, mIconType=0, pkgName=co.realfit.agdkwinitwgpu, className=co.realfit.agdkwinitwgpu.MainActivity, screenId=4, container=-100, cellX=0, cellY=1, spanX=1, spanY=1, isLandscapePos=false
05-21 23:34:03.743  1602  1627 I ActivityTaskManager: START u0 {act=android.intent.action.MAIN cat=[android.intent.category.LAUNCHER] flg=0x10200000 cmp=co.realfit.agdkwinitwgpu/.MainActivity bnds=[40,325][144,429] (has extras)} from uid 10086
05-21 23:34:03.772  1602  1652 W ActivityManager: ProcessRecord{cb45348 0:co.realfit.agdkwinitwgpu/u0a296}: ART verification disabled
05-21 23:34:03.797   527   527 D OemNetd : setPidForPackage: packageName=co.realfit.agdkwinitwgpu, pid=15779, pid=10296
05-21 23:34:03.797  1602  1671 I ActivityManager: Start proc 15779:co.realfit.agdkwinitwgpu/u0a296 for activity {co.realfit.agdkwinitwgpu/co.realfit.agdkwinitwgpu.MainActivity} caller=com.miui.home
05-21 23:34:03.922  5050 12893 D PerfEngineController: ForegroundInfo{mForegroundPackageName='co.realfit.agdkwinitwgpu', mForegroundUid=10296, mForegroundPid=15779, mLastForegroundPackageName='com.miui.home', mLastForegroundUid=10086, mLastForegroundPid=2371, mMultiWindowForegroundPackageName='null', mMultiWindowForegroundUid=-1, mFlags=1}
05-21 23:34:03.922  4112 10742 I ProcessMonitor: onForegroundInfoChanged: ForegroundInfo{mForegroundPackageName='co.realfit.agdkwinitwgpu', mForegroundUid=10296, mForegroundPid=15779, mLastForegroundPackageName='com.miui.home', mLastForegroundUid=10086, mLastForegroundPid=2371, mMultiWindowForegroundPackageName='null', mMultiWindowForegroundUid=-1, mFlags=1}
05-21 23:34:03.922  6492  6508 D AutoInstallService: onForegroundInfoChanged: co.realfit.agdkwinitwgpu
05-21 23:34:03.922  5073  5097 I GST     : forePkg: co.realfit.agdkwinitwgpu, preForePkg: com.miui.home
05-21 23:34:03.923  4112 10742 I GameBoosterService: onForegroundInfoChanged: Cur=co.realfit.agdkwinitwgpu       last=com.miui.home
05-21 23:34:03.923  4112 10742 D GameBoosterService: onGameStatusChange foreground:ForegroundInfo{mForegroundPackageName='co.realfit.agdkwinitwgpu', mForegroundUid=10296, mForegroundPid=15779, mLastForegroundPackageName='com.miui.home', mLastForegroundUid=10086, mLastForegroundPid=2371, mMultiWindowForegroundPackageName='null', mMultiWindowForegroundUid=-1, mFlags=1}
05-21 23:34:03.953  6365  6365 D TouchAssistant: getPackageNameFromPid packageName:co.realfit.agdkwinitwgpu
05-21 23:34:03.953  6365  6365 D TouchAssistant: onTopAppChanged packageName:co.realfit.agdkwinitwgpu
05-21 23:34:04.154 15779 15779 D ForceDarkHelper: updateByCheckExcludeList: pkg: co.realfit.agdkwinitwgpu activity: co.realfit.agdkwinitwgpu.MainActivity@caad8f4
05-21 23:34:04.159 15779 15779 D ForceDarkHelper: updateByCheckExcludeList: pkg: co.realfit.agdkwinitwgpu activity: co.realfit.agdkwinitwgpu.MainActivity@caad8f4
05-21 23:34:04.166 15779 15779 D ForceDarkHelper: updateByCheckExcludeList: pkg: co.realfit.agdkwinitwgpu activity: co.realfit.agdkwinitwgpu.MainActivity@caad8f4
05-21 23:34:04.174 15779 15779 D ForceDarkHelper: updateByCheckExcludeList: pkg: co.realfit.agdkwinitwgpu activity: co.realfit.agdkwinitwgpu.MainActivity@caad8f4
05-21 23:34:04.338 15779 15804 D vulkan  : searching for layers in '/data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/lib/arm64'
05-21 23:34:04.339 15779 15804 D vulkan  : searching for layers in '/data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk!/lib/arm64-v8a'
05-21 23:34:04.413 15779 15804 D vulkan  : added global layer 'VK_LAYER_KHRONOS_validation' from library '/data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk!/lib/arm64-v8a/libVkLayer_khronos_validation.so'
05-21 23:34:04.686 15779 15804 I chatty  : uid=10296(co.realfit.agdkwinitwgpu) identical 22 lines
05-21 23:34:04.687 15779 15804 I chatty  : uid=10296(co.realfit.agdkwinitwgpu) identical 10 lines
05-21 23:34:04.687 15779 15804 I chatty  : uid=10296(co.realfit.agdkwinitwgpu) identical 2 lines
05-21 23:34:04.687 15779 15804 I chatty  : uid=10296(co.realfit.agdkwinitwgpu) identical 2 lines
05-21 23:34:04.688 15779 15804 I chatty  : uid=10296(co.realfit.agdkwinitwgpu) identical 10 lines
05-21 23:34:04.775 15821 15821 F DEBUG   : pid: 15779, tid: 15804, name: t.agdkwinitwgpu  >>> co.realfit.agdkwinitwgpu <<<
05-21 23:34:04.781 15821 15821 F DEBUG   :       #01 pc 00000000004cfe0c  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #02 pc 00000000004cd274  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #03 pc 00000000004cd118  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #04 pc 00000000004cce04  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #05 pc 00000000004cb1e0  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #06 pc 00000000004ccb78  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #07 pc 000000000002d3a8  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #08 pc 00000000004912c0  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #09 pc 0000000000493cec  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #10 pc 0000000000031530  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #11 pc 000000000002f3f4  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #12 pc 000000000002f1b0  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #13 pc 0000000000032548  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #14 pc 000000000002ef74  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #15 pc 000000000003d160  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #16 pc 0000000000033384  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #17 pc 0000000000034af8  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #18 pc 0000000000041284  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #19 pc 000000000003f728  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.781 15821 15821 F DEBUG   :       #20 pc 000000000004391c  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.782 15821 15821 F DEBUG   :       #21 pc 00000000000418bc  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.782 15821 15821 F DEBUG   :       #22 pc 000000000003f680  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.782 15821 15821 F DEBUG   :       #23 pc 000000000004374c  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.782 15821 15821 F DEBUG   :       #24 pc 000000000003fdbc  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:04.782 15821 15821 F DEBUG   :       #25 pc 000000000003fe28  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000) (android_main+60)
05-21 23:34:04.782 15821 15821 F DEBUG   :       #26 pc 0000000000497a20  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000) (_rust_glue_entry+348)
05-21 23:34:04.782 15821 15821 F DEBUG   :       #27 pc 00000000004ae050  /data/app/co.realfit.agdkwinitwgpu-1UaSMxVxaOLgOB-XONGtEg==/base.apk (offset 0x2394000)
05-21 23:34:05.492  1602 15824 W ActivityTaskManager:   Force finishing activity co.realfit.agdkwinitwgpu/.MainActivity
05-21 23:34:05.598  1602  2436 I ActivityManager: Process co.realfit.agdkwinitwgpu (pid 15779) has died: vis+99 TOP
05-21 23:34:05.613  5050 12893 D PerfEngineController: ForegroundInfo{mForegroundPackageName='com.miui.home', mForegroundUid=10086, mForegroundPid=2371, mLastForegroundPackageName='co.realfit.agdkwinitwgpu', mLastForegroundUid=10296, mLastForegroundPid=15779, mMultiWindowForegroundPackageName='null', mMultiWindowForegroundUid=-1, mFlags=0}
05-21 23:34:05.613  4112 10742 I ProcessMonitor: onForegroundInfoChanged: ForegroundInfo{mForegroundPackageName='com.miui.home', mForegroundUid=10086, mForegroundPid=2371, mLastForegroundPackageName='co.realfit.agdkwinitwgpu', mLastForegroundUid=10296, mLastForegroundPid=15779, mMultiWindowForegroundPackageName='null', mMultiWindowForegroundUid=-1, mFlags=0}
05-21 23:34:05.614  5073  5097 I GST     : forePkg: com.miui.home, preForePkg: co.realfit.agdkwinitwgpu
05-21 23:34:05.614  4112 10742 I GameBoosterService: onForegroundInfoChanged: Cur=com.miui.home  last=co.realfit.agdkwinitwgpu
05-21 23:34:05.615  4112 10742 D GameBoosterService: onGameStatusChange foreground:ForegroundInfo{mForegroundPackageName='com.miui.home', mForegroundUid=10086, mForegroundPid=2371, mLastForegroundPackageName='co.realfit.agdkwinitwgpu', mLastForegroundUid=10296, mLastForegroundPid=15779, mMultiWindowForegroundPackageName='null', mMultiWindowForegroundUid=-1, mFlags=0}
05-21 23:34:05.993  1602  1652 W ActivityTaskManager: Activity top resumed state loss timeout for ActivityRecord{eecd865 u0 co.realfit.agdkwinitwgpu/.MainActivity t-1 f}
```
