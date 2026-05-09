import 'dart:async';
import 'package:flutter/material.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const ZingerBoostApp());
}

class ZingerBoostApp extends StatefulWidget {
  const ZingerBoostApp({super.key});
  static _ZingerBoostAppState? of(BuildContext ctx) => ctx.findAncestorStateOfType<_ZingerBoostAppState>();
  @override State<ZingerBoostApp> createState() => _ZingerBoostAppState();
}

class _ZingerBoostAppState extends State<ZingerBoostApp> {
  ThemeMode _theme = ThemeMode.dark;
  void toggleTheme() => setState(() => _theme = _theme == ThemeMode.dark ? ThemeMode.light : ThemeMode.dark);
  @override Widget build(BuildContext c) => MaterialApp(title:'ZingerBoost', debugShowCheckedModeBanner:false, theme:ThemeData.light(useMaterial3:true), darkTheme:ThemeData.dark(useMaterial3:true).copyWith(scaffoldBackgroundColor:const Color(0xFF0A0A0A), cardColor:const Color(0xFF171717)), themeMode:_theme, home:const Shell());
}

const _pages = [Dash(), Tweaks(), Services(), Cleaner(), Snaps(), Debloat(), Software(), Sett()];
const _titles = ['Dashboard','Tweaks','Services','Cleaner','Snapshots','Debloat','Software','Settings'];
const _icons = [Icons.dashboard,Icons.tune,Icons.settings,Icons.cleaning_services,Icons.history,Icons.delete_forever,Icons.download,Icons.palette];

class Shell extends StatefulWidget { const Shell({super.key}); @override State<Shell> createState() => _ShellState(); }
class _ShellState extends State<Shell> { int i=0; @override Widget build(BuildContext c)=>Scaffold(body:Row(children:[NavigationRail(selectedIndex:i,onDestinationSelected:(v)=>setState(()=>i=v),labelType:NavigationRailLabelType.all,backgroundColor:Theme.of(c).brightness==Brightness.dark?const Color(0xFF171717):Colors.grey.shade100,selectedIconTheme:const IconThemeData(color:Color(0xFF0EA5E9)),destinations:List.generate(8,(x)=>NavigationRailDestination(icon:Icon(_icons[x]),label:Text(_titles[x])))),const VerticalDivider(width:1),Expanded(child:Column(crossAxisAlignment:CrossAxisAlignment.start,children:[Padding(padding:const EdgeInsets.all(16),child:Text(_titles[i],style:const TextStyle(fontSize:22,fontWeight:FontWeight.bold))),Expanded(child:_pages[i])]))]));}

// ---- PAGES ----

class Dash extends StatelessWidget { const Dash({super.key}); @override Widget build(c)=>const Center(child:Text('Dashboard')); }
class Tweaks extends StatelessWidget { const Tweaks({super.key}); @override Widget build(c)=>const Center(child:Text('Tweaks')); }
class Services extends StatelessWidget { const Services({super.key}); @override Widget build(c)=>const Center(child:Text('Services')); }
class Cleaner extends StatelessWidget { const Cleaner({super.key}); @override Widget build(c)=>const Center(child:Text('Cleaner')); }
class Snaps extends StatelessWidget { const Snaps({super.key}); @override Widget build(c)=>const Center(child:Text('Snapshots')); }
class Debloat extends StatelessWidget { const Debloat({super.key}); @override Widget build(c)=>const Center(child:Text('Debloat')); }
class Software extends StatelessWidget { const Software({super.key}); @override Widget build(c)=>const Center(child:Text('Software')); }
class Sett extends StatelessWidget { const Sett({super.key}); @override Widget build(c)=>const Center(child:Text('Settings')); }
