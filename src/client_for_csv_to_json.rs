pub mod module_csv_to_json {
    use actix_web::client::{Client, Connector};
    use openssl::ssl::{SslConnector, SslMethod};
    use encoding_rs;
    use serde_json;
    use csv;
    use std::collections::HashMap;

    pub async fn get_csv_to_json() -> String {
      let builder = SslConnector::builder(SslMethod::tls()).unwrap();
      let client = Client::builder()
                    .connector(Connector::new().ssl(builder.build()).finish())
                    .finish();
      let result = client
                    // csvファイル：北海道の新型コロナウイルス感染症に関するデータ
                    .get("https://www.harp.lg.jp/opendata/dataset/1369/resource/2828/patients.csv")
                    .send()
                    .await
                    .unwrap()
                    .body()
                    .limit(20000000) // スキームタイプ毎の最大同時接続数
                    .await
                    .unwrap();


        // csvのデコード処理
        let (result_decode, _, _) = encoding_rs::SHIFT_JIS.decode(&result);
        let rchange = result_decode.into_owned();
        let mut rdr = csv::Reader::from_reader(rchange.as_bytes());

        // csvデータを一行毎HashMapに収納
        let mut csv_hash = HashMap::new();
        // 全てのcsvを収納(行毎に収納したHashMapをVecに収納)
        let mut csv_vec_hash = Vec::new();

        for rc in rdr.records() {
          // csv一行毎のデータ
          let rc_c = rc.unwrap().clone();
                
          // csvデータをHashMapに保存
          csv_hash.insert("No", rc_c.get(0).unwrap().to_owned());
          csv_hash.insert("リリース日", rc_c.get(1).unwrap().to_owned());
          csv_hash.insert("曜日", rc_c.get(2).unwrap().to_owned());
          csv_hash.insert("居住地", rc_c.get(3).unwrap().to_owned());
          csv_hash.insert("年代", rc_c.get(4).unwrap().to_owned());
          csv_hash.insert("性別", rc_c.get(5).unwrap().to_owned());
          csv_hash.insert("属性", rc_c.get(6).unwrap().to_owned());
          csv_hash.insert("備考", rc_c.get(7).unwrap().to_owned());
          csv_hash.insert("補足", rc_c.get(8).unwrap().to_owned());
          csv_hash.insert("退院", rc_c.get(9).unwrap().to_owned());
          csv_hash.insert("周囲の状況", rc_c.get(10).unwrap().to_owned());
          csv_hash.insert("濃厚接触者の状況", rc_c.get(11).unwrap().to_owned());
          csv_hash.insert("age_group", rc_c.get(12).unwrap().to_owned());
          csv_hash.insert("sex_en", rc_c.get(13).unwrap().to_owned());
        
          // HAshMapをVecに保存
          csv_vec_hash.push(csv_hash.clone());
        }

        // data形式を揃える為ルートにdata追加
        let mut json_data = HashMap::new();
        json_data.insert("data", csv_vec_hash);
            
        // ハッシュマップをjson形式の文字列に変換
        serde_json::to_string(&json_data).unwrap()
    }
}