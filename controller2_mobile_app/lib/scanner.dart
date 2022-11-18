
import "dart:io";
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:mobile_scanner/mobile_scanner.dart' as mb;
import 'package:qr_code_scanner/qr_code_scanner.dart';
class Scanner extends StatefulWidget {
  final int mode;
  const Scanner({Key? key, required this.mode}) : super(key: key);

  @override
  _ScannerState createState() => _ScannerState();
}

class _ScannerState extends State<Scanner> {
  final GlobalKey qrKey = GlobalKey(debugLabel: 'QR');
  late QRViewController controller;
  var scannedData = 'Scan a code';
  mb.Barcode? result;
  late int mode;

  @override
  void initState() {
    mode = widget.mode;
    super.initState();
  }

  // @override
  // void reassemble() {
  //   super.reassemble();
  //   if (Platform.isAndroid) {
  //     controller.pauseCamera();
  //   } else if (Platform.isIOS) {
  //     controller.resumeCamera();
  //   }
  // }

  @override
  void dispose() {
    //controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Scanner"),
      ),
      body: Stack(
        children: [
          Column(
            children: <Widget>[
              Expanded(
                flex: 5,
                child: Stack(
                  children: [
                    mb.MobileScanner(
                      allowDuplicates: false,
                      onDetect: (barcode, args){
                        print(barcode.rawValue);
                        if(mode ==1){
                          if(barcode.rawValue!.contains("eid") && barcode.rawValue!.contains("scheme")){
                            setState((){
                              result = barcode;
                            });
                          }
                        }
                        if(mode ==2){
                          if(barcode.rawValue!.contains("cid") && barcode.rawValue!.contains("role")){
                            setState((){
                              result = barcode;
                            });
                          }
                        }
                        if(mode ==3){
                          if(barcode.rawValue!.contains("issuer") && barcode.rawValue!.contains("data")){
                            setState((){
                              result = barcode;
                            });
                          }
                        }
                      },
                    ),
                    // QRView(
                    //   key: qrKey,
                    //   onQRViewCreated: _onQRViewCreated,
                    // ),
                    Center(
                      child: Container(
                        width: 300,
                        height: 300,
                        decoration: BoxDecoration(
                          border: Border.all(
                            color: result != null ? Colors.green : Colors.red,
                            width: 4,
                          ),
                          borderRadius: BorderRadius.circular(12),
                        ),
                      ),
                    )
                  ],
                ),
              ),
              Expanded(
                flex: 5,
                child: SingleChildScrollView(
                  child: Center(
                    child: (result != null)
                        ? Column(
                      children: [
                        Text( mode == 1 ? 'Watcher oobi: ${result!.rawValue}' : mode == 2 ? 'Issuer oobi: ${result!.rawValue}' : mode == 3 ? 'ACDC: ${result!.rawValue}' :
                         'Incorrect mode'),
                        RawMaterialButton(
                            onPressed: () {
                              Navigator.pop(context, result!.rawValue);
                            },
                            child: Text("Accept", style: TextStyle(fontWeight: FontWeight.bold, color: Colors.green),),
                            shape: RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(18.0),
                                side: BorderSide(width: 2, color: Colors.green)
                            )
                        ),
                      ],
                    )
                        : Text('Scan a code'),
                  ),
                ),
              )
            ],
          ),
        ],
      ),
    );
  }

  // void _onQRViewCreated(QRViewController controller) {
  //   this.controller = controller;
  //   controller.scannedDataStream.listen((scanData) {
  //     setState(() {
  //       if(mode ==1){
  //         if(scanData.code!.contains("eid") && scanData.code!.contains("scheme")){
  //           result = scanData;
  //         }
  //       }
  //       if(mode ==2){
  //         if(scanData.code!.contains("cid") && scanData.code!.contains("role")){
  //           result = scanData;
  //         }
  //       }
  //       if(mode ==3){
  //         if(scanData.code!.contains("issuer") && scanData.code!.contains("data")){
  //           result = scanData;
  //         }
  //       }
  //     });
  //   });
  // }
}