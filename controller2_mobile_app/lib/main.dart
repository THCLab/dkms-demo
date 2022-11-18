import 'dart:async';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';
import 'dart:convert';

import 'package:asymmetric_crypto_primitives/asymmetric_crypto_primitives.dart';
import 'package:asymmetric_crypto_primitives/ed25519_signer.dart';
import 'package:dkms_demo/scanner.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:keri/bridge_generated.dart';
import 'package:keri/keri.dart';
import 'package:path_provider/path_provider.dart';
import 'package:vector_math/vector_math.dart';



void main() async{
  WidgetsFlutterBinding.ensureInitialized();
  var signer = await AsymmetricCryptoPrimitives.establishForEd25519();
  runApp(MaterialApp(home: MyApp(signer: signer,),debugShowCheckedModeBanner: false,));
}

class MyApp extends StatefulWidget {
  final Ed25519Signer signer;
  const MyApp({Key? key, required this.signer}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  var platform = const MethodChannel('samples.flutter.dev/getkey');
  var current_b64_key='';
  var next_b64_key='';
  var watcher_oobi ='';
  var issuer_oobi ='';
  var icp_event;
  var hex_signature = '';
  var hex_sig = '';
  var signature;
  var controller;
  var kel;
  var isVerified;
  var key_sig_pair;
  var toVerify = '';
  var add_watcher_message;
  var parsedAttachment;
  var acdc ='';
  late Ed25519Signer signer;


  @override
  void initState() {
    signer = widget.signer;
    initParameters();
    super.initState();
  }

  Future<bool> _verify(String message, String signature, String key) async{
    var result = await platform.invokeMethod('verify', {'message': message, 'signature': signature, 'key' : key});
    return result;
  }

  Future<void> initParameters() async{
    String dbPath = await getLocalPath();
    dbPath = dbPath + '/new';
    current_b64_key = await signer.getCurrentPubKey();
    next_b64_key = await signer.getNextPubKey();
    await Keri.initKel(inputAppDir: dbPath);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SingleChildScrollView(
        child: Column(
          children: [
            const SizedBox(height: 80,),
            Text('1. Scan for watcher oobi:', style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 20),),
            //Text(attachment),
            RawMaterialButton(
                onPressed: () async{
                  watcher_oobi = await Navigator.push(
                    context,
                    MaterialPageRoute(builder: (context) => const Scanner(mode: 1,)),
                  );
                  setState(() {

                  });
                  List<PublicKey> vec1 = [];
                  vec1.add(await Keri.newPublicKey(kt: KeyType.Ed25519, keyB64: current_b64_key));
                  List<PublicKey> vec2 = [];
                  vec2.add(await Keri.newPublicKey(kt: KeyType.Ed25519, keyB64: next_b64_key));
                  List<String> vec3 = [];
                  print("incept keys: ${vec1[0].publicKey}, ${vec2[0].publicKey}");
                  icp_event = await Keri.incept(publicKeys: vec1, nextPubKeys: vec2, witnesses: vec3, witnessThreshold: 0);
                  hex_signature = await signer.sign(icp_event);
                  print("Hex signature: $hex_signature");

                  //Sign icp event
                  signature = await Keri.signatureFromHex(st: SignatureType.Ed25519Sha512, signature: hex_signature);

                  controller = await Keri.finalizeInception(event: icp_event, signature: signature);
                  kel = await Keri.getKel(cont: controller);
                  print("Current controller kel: $kel");

                  add_watcher_message = await Keri.addWatcher(controller: controller, watcherOobi: watcher_oobi);
                  print("\nController generate end role message to add witness: $add_watcher_message");

                  hex_sig = await signer.sign(add_watcher_message);
                  signature = await Keri.signatureFromHex(st: SignatureType.Ed25519Sha512, signature: hex_sig);
                  print("end role message signature: $hex_sig");

                  await Keri.finalizeEvent(identifier: controller, event: add_watcher_message, signature: signature);
                },
                child: const Text("Scan", style: TextStyle(fontWeight: FontWeight.bold),),
                shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(18.0),
                    side: const BorderSide(width: 2)
                )
            ),
            Text(watcher_oobi.toString()),
            const SizedBox(height: 20,),
            watcher_oobi.isNotEmpty ? const Text('2. Scan for issuer oobi:', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 20),) : Container(),
            watcher_oobi.isNotEmpty ? RawMaterialButton(
                onPressed: () async{
                  issuer_oobi = await Navigator.push(
                    context,
                    MaterialPageRoute(builder: (context) => const Scanner(mode: 2,)),
                  );
                  print("\nSending issuer oobi to watcher: $issuer_oobi");
                  print("Querying abour issuer kel...");
                  //await api.query(controller: controller, oobisJson: issuer_oobi);
                  setState(() {

                  });
                },
                child: const Text("Scan", style: TextStyle(fontWeight: FontWeight.bold),),
                shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(18.0),
                    side: const BorderSide(width: 2)
                )
            ) : Container(),
            Text(issuer_oobi.toString()),
            const SizedBox(height: 20,),
            issuer_oobi.isNotEmpty ? const Text('3. Scan for ACDC:', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 20),) : Container(),
            issuer_oobi.isNotEmpty ? RawMaterialButton(
                onPressed: () async{
                  acdc = await Navigator.push(
                    context,
                    MaterialPageRoute(builder: (context) => const Scanner(mode: 3,)),
                  );
                  setState(() {
                    isVerified = null;
                  });
                  var splitAcdc = acdc.split('-FAB');
                  print(splitAcdc);
                  var attachmentStream = '-FAB' + splitAcdc[1];
                  toVerify = splitAcdc[0];
                  print(attachmentStream);
                  key_sig_pair = await Keri.getCurrentPublicKey(attachment: attachmentStream);
                  print(key_sig_pair);
                },
                child: const Text("Scan", style: TextStyle(fontWeight: FontWeight.bold),),
                shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(18.0),
                    side: const BorderSide(width: 2)
                )
            ) : Container(),
            Text(acdc),
            const SizedBox(height: 20,),
            acdc.isNotEmpty ? const Text('4. Verify ACDC:', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 20),) : Container(),
            acdc.isNotEmpty ? RawMaterialButton(
                onPressed: () async{
                  print(key_sig_pair[0].signature.key.toString());
                  print(key_sig_pair[0].key.key.toString());
                  isVerified = await _verify(toVerify.toString(), key_sig_pair[0].signature.key.toString(), key_sig_pair[0].key.key.toString());
                  setState(() {

                  });
                },
                child: const Text("Verify", style: TextStyle(fontWeight: FontWeight.bold),),
                shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(18.0),
                    side: const BorderSide(width: 2)
                )
            ) : Container(),
            isVerified != null ? (isVerified ? const Text("Verification successful", style: TextStyle(color: Color(0xff21821e)),) : const Text("Verification error", style: TextStyle(color: Color(0xff781a22)),)): Container(),
          ],
        ),
      ),
    );
  }

  List<String> splitMessage(String message){
    return message.split("}-");
  }

  Future<String> getLocalPath() async {
    final directory = await getApplicationDocumentsDirectory();
    return directory.path;
  }


}