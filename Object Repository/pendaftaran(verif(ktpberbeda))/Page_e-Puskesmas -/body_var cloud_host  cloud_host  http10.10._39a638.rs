<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_var cloud_host  cloud_host  http10.10._39a638</name>
   <tag></tag>
   <elementGuidId>e0f426e6-810f-4a96-911b-6bd4ed2b77d2</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>ac38b079-d7e4-44e4-a641-c044c55eb0bb</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                            
	var cloud_host = &quot;&quot;;
									cloud_host = 'http://10.10.10.10/home';
				localStorage.setItem(&quot;epuskesmas-cloud-server&quot;, cloud_host);

	
	    #connection {
	        position: fixed;
	        font-weight: bold;
	        bottom: 5px;
	        left: 5px;
	        z-index: 99;
	        font-size: 16px;
	        border: none;
	        outline: none;
	        color: white;
	        cursor: pointer;
	        padding: 3px;
	        border-radius: 4px;
	    }
	    .online {
	      background: green;
	    }
	    .offline {
	      background: red;
	    }
	
	
	
	    window.onbeforeunload = function(event){
	    	var connection = document.getElementById('connection').innerHTML;
	    	localStorage.setItem('epuskesmas-connection-status', connection);
	    }
	    if ('serviceWorker' in navigator) {
	    	document.onreadystatechange = function(e){
	    		if(localStorage.getItem('epuskesmas-connection-status') != null &amp;&amp; localStorage.getItem('epuskesmas-connection-status').length > 0){
		            document.getElementById('connection').innerHTML = localStorage.getItem('epuskesmas-connection-status');
		            document.getElementById('connection').className = localStorage.getItem('epuskesmas-connection-status').toLowerCase();
	    		}
			}

	        window.addEventListener('load', function () {
	            navigator.serviceWorker.register('/service-worker.js').then(function(worker){
		    		if(localStorage.getItem('epuskesmas-connection-status') != null &amp;&amp; localStorage.getItem('epuskesmas-connection-status').length > 0){
		    			var loadConnection = localStorage.getItem('epuskesmas-connection-status').toLowerCase();
									                if(nextConnection == 'online'){
			            		sweetAlert('online');
			                }else{
			            		sweetAlert('offline', 'warning');
			                }
								    			sweetAlert(localStorage.getItem('epuskesmas-connection-status').toLowerCase());
		    		}
	            });
	        });
	        
	        function send_message_to_service_worker(messages){
	            return new Promise(function(resolve, reject){
	                var messageChannel = new MessageChannel();
	                messageChannel.port1.onmessage = function(event){
	                    if(event.data.error){
	                        reject(event.data.error);
	                    }else{
	                        resolve(event.data);
	                    }
	                };
	                navigator.serviceWorker.controller.postMessage(messages, [messageChannel.port2]);
	            });
	        }

			navigator.serviceWorker.getRegistrations().then(registrations => {
				registrations.forEach(registration => {
					registration.unregister()
				})
			})

	        navigator.serviceWorker.addEventListener('message', function(event){
	        	if(event.data.type == 'check connection'){
		            var previousConnection = document.getElementById('connection').className;
		            if(event.data.status != &quot;&quot;){
			            document.getElementById('connection').innerHTML = event.data.status;
			            document.getElementById('connection').className = event.data.status.toLowerCase();
		            }
		            var nextConnection = document.getElementById('connection').className;
								            if(previousConnection == 'offline'){
			                if(nextConnection == 'online'){
			            		sweetAlert('online');
			                }else{
			            		sweetAlert('offline', 'warning');
			                }
			            }else{
			                if(previousConnection == 'online'){
			                	if(nextConnection == 'offline'){
				            		sweetAlert('offline');
			                	}
			                }
			            }
						        	}
	        });

	        function sweetAlert(message = null, timer = 3000){
						        	switch(message) {
					  	case 'offline':
		                	Swal.fire({
		                       	html: '&lt;h5>ePuskesmas NG mendeteksi adanya gangguan pada koneksi internet.&lt;br>Pastikan perangkat anda terhubung dengan koneksi internet.&lt;h5>',
								imageUrl: 'https://demo.epuskesmas.id/images/o2o/offline.png',
								imageWidth: 400,
								imageHeight: 175,
								allowOutsideClick: false,
								showConfirmButton: &quot;1&quot;,
		                       	confirmButtonColor: '#224d09',
		                       	confirmButtonText: 'Ya, Alihkan ke ePuskesmas NG Offline!'
		                   	}).then((result) => {
		                       	if (result.value) {
		                        	if(cloud_host == localStorage.getItem('epuskesmas-cloud-server')){
		                      			window.location = localStorage.getItem('epuskesmas-cloud-server');
		                        	}
		                       	}
		                   	})
					    	break;
		        		default:
			  				Swal.fire({
		                        html: '&lt;h5>ePuskesmas NG telah terhubung (online).&lt;h5>',
								imageUrl: 'https://demo.epuskesmas.id/images/o2o/online.png',
								imageWidth: 400,
								imageHeight: 175,
		                        timer: timer,
								showConfirmButton: false
		                    });
					}
					        }

	        setInterval(function(){
	            send_message_to_service_worker({'type' : 'check connection', 'url' :''});
	        }, 8000);
	    }
	
                
        
            
                
                
                    
                        Toggle navigation
                        
                    
                
                
                                                                   
                                                                                                
                                        Pendaftaran 
                                        
                                                                                        Pasien &amp; KK
                                                                                                                                    Pendaftaran Pasien
                                                                                                                                    Rekam Medis
                                                                                                                                    Antrean
                                                                                         
                                            Panggil Antrean
                                                                                    
                                    
                                
                                                                    
                                        Pelayanan 
                                        
                                                                                            Pelayanan
                                                                                                Medis
                                                                                                                                                Resep
                                                                                                                                                Obat Pasien
                                                                                                                                                Penimbangan Balita
                                                                                                                                                Diare Advokasi
                                                                                                                                                Program Puskesmas
                                                                                                                                        
                                                                                            Pelayanan Luar Gedung
                                                                                                    Skrining Anak Sekolah
                                                                                                                                                   Skrining Santri
                                                                                                                                                    Skrining PTM
                                                                                                                                                    Sinkronisasi Kunjungan Sehat BPJS
                                                                                                                                                    Sinkronisasi Data Sihepi
                                                                                            
                                                                                            Pemeriksaan
                                                                                                Laboratorium
                                                                                                                                            
                                                                                            Pasien Pulang
                                                                                                Rujukan External
                                                                                               
                                                                                                Pasien Meninggal
                                                                                                                                                Pembayaran
                                                                                                                                                Detail Transaksi
                                                                                            
                                                                                            Survey Kepuasan/Testimoni Pasien
                                                                                                Survey Kepuasan &amp; Testimoni Pasien
                                                                                                                                                Pengelolaan Survey Kepuasan &amp; Testimoni Pasien
                                                                                            
                                                                                            Lroa
                                                                                                Lroa
                                                                                                                                    
                                    
                                                            
                                                            
                                    Pengelolaan 
                                    
                                                                                    Gudang Farmasi
                                                                                        Pemesanan Obat
                                                                                                                                    Penerimaan Obat
                                                                                                                                    Stok Obat
                                                                                                                                    Stok Opname
                                                                                                                                    Distribusi Obat
                                                                                    
                                                                                    Data Master
                                                                                        Pegawai
                                                                                                                                    Pengguna
                                                                                                                                                                                Ruangan
                                                                                                                                    Ruangan Akses
                                                                                                                                    Ruangan Puskesmas
                                                                                                                                    Ruangan JAKESH
                                                                                                                                    Puskesmas
                                                                                                                                    Perlengkapan
                                                                                                                                    Sasaran Program
                                                                                                                                                                                            Sasaran Proyeksi
                                                                                                                                                                                    Jadwal Pelayanan
                                                                                                                                    Jadwal Shift
                                                                                                                                    Libur
                                                                                                                                    Data Tindakan
                                                                                                                                    Data Imunisasi
                                                                                                                                    Kamar &amp; Bed
                                                                                                                                    Sekolah
                                                                                                                                    Asuransi
                                                                                                                                    Laboratorium
                                                                                                                                    Paket Laboratorium
                                                                                                                                    Obat
                                                                                                                                    Konfigurasi Dashboard Antrian
                                                                                                                                    Konfigurasi Form
                                                                                                                                    Konfigurasi Laboratorium
                                                                                                                                    SMS Konfigurasi
                                                                                                                                    Konfigurasi Pembulatan
                                                                                                                                    Asupan Makanan
                                                                                                                                    Industri
                                                                                                                                    PBF
                                                                                                                                                                                Komponen Tarif
                                                                                                                                    Metode Pembayaran
                                                                                                                                                                                Diagnosa
                                                                                                                                    Template Print
                                                                                                                                                                      Data Dasar Puskesmas
                                        
                                                                                    BPJS
                                                                                        Pengaturan BPJS
                                                                                                                                    RS Provider BPJS
                                                                                                                                    Poli FKTL BPJS
                                                                                                                                    Dokter BPJS
                                                                                                                                    Tindakan BPJS
                                                                                                                                    Laboratorium BPJS
                                                                                                                                    Obat BPJS
                                                                                                                                    Ruangan BPJS
                                                                                                                                    Pengaturan Bridging BPJS
                                                                                                                                    Sinkron BPJS
                                                                                                                                    Sinkron BPJS Jejaring
                                                                                                                                
                                        

                                        
                                                                                    SISRUTE
                                                                                        Konfigurasi
                                                                                                                                    Tujuan Rujukan
                                                                                                                                    Kriteria Khusus
                                                                                       
                                                                                        SDM
                                                                                                                                    Ruang Perawatan
                                                                                                                                    Pelayanan
                                                                                                                                    Sarana
                                                                                                                                    Kelas Perawatan
                                                                                                                                    Alkes
                                                                                    
                                                                                    Master Wilayah
                                                                                        Wilayah Kerja
                                                                                                                                    Wilayah Kerja Posyandu
                                                                                    
                                                                                    SIPTM
                                                                                        Puskesmas
                                                                                                                                    Diagnosis
                                                                                                                                    Sinkron SIPTM
                                                                                    
                                        
                                                                                    Scan
                                                                                        Scan QRCode
                                                                                                                                                                        Antrian
                                                                                        Antrian
                                                                                                                                                                        Speaker
                                                                                        Speaker
                                                                                                                                                                
                                
                            
                                                            
                                    GIS 
                                    
                                                                                Pasien
                                                                                                                        Penyakit
                                                                            
                                
                                                                                        
                                    Laporan 
                                    
                                                                                    Grafik
                                            Grafik
                                                                                                                            Dashboard SIP
                                        
                                                                                    Monitoring
                                                                                                                                    Penggunaan Fitur
                                                                                    
                                                                                    Laporan Harian
                                                                                                                                    Pendapatan Tindakan
                                                                                                                                    Kunjungan Pasien
                                                                                                                                    Kunjungan Pasien BPJS
                                                                                                                                    Pelayanan Pasien
                                                                                                                                    Pemeriksaan Medis
                                                                                                                                    Pelayanan Resep
                                                                                                                                    Pengeluaran Obat Pasien
                                                                                                                                    Pengeluaran Alat Kesehatan
                                                                                                                                    Pelayanan Laboratorium
                                                                                                                                    Rujukan Internal
                                                                                                                                    Rujukan External
                                                                                                                                    
                                                                                                    Tindakan Dokter/Perawat
                                                                                            
                                                                                                                                    Pemeriksaan Laboratorium
                                                                                                                                    Kinerja Puskesmas
                                                                                                                                    Pelayanan Ruangan
                                                                                                                                    Penyakit
                                                                                                                                    Asuhan Keperawatan
                                                                                                                                    Kunjungan PTM
                                                                                                                                    PKPR
                                                                                                                                    Alat Kesehatan Terbanyak
                                                                                                                                    Tindakan Terbanyak
                                                                                                                                    Obat Kadaluarsa
                                                                                                                                    Laporan Pendapatan
                                                                                                                                    Laporan Pendapatan Pendaftaran 
                                             
                                                                                                                                                                                        Laporan Pendapatan Kasir
                                                                                                                                                                                    Laporan Transaksi Kasir
                                                                                                                                                                                Laporan Skrining COVID-19
                                                                                                                                    Laporan Pengeluaran Obat
                                                                                                                                    Laporan Stok Opname
                                                                                                                                        Laporan Jasa Tenaga Medis
                                                                                                                                        Laporan Hepatitis C
                                                                                                                                        Laporan Diare
                                                                                                                                        Laporan Hepatitis B
                                                                                                                                        Pendapatan Laboratorium
                                                                                    
                                                                                    Laporan Mingguan
                                                                                        Laporan PWS Penyakit
                                                                                    
                                                                                    Laporan Bulanan
                                                                                        Laporan
                                                                                                                                    Data Dasar
                                                                                                                                    Kirim Report
                                                                                                                                    Monitoring Laporan
                                                                                                                                    SP3 LB1
                                                                                                                                    SP3 LB2 (LPLPO)
                                                                                                                                    SP3 LB3
                                                                                                                                    SP3 LB4
                                                                                                                                    UKP-1. Pelayanan Puskesmas
                                                                                                                                    UKP-2. Kesakitan Umum
                                                                                                                                    UKP-3. Kesakitan Gigi dan Mulut
                                                                                                                                    UKP-4. Data Kesakitan Terbanyak
                                                                                                                                    UKP-5. Data Kematian di Puskesmas
                                                                                                                                    UKP-6. LPLPO
                                                                                                                                    UKME-1. Promosi Kesehatan
                                            UKME-2. Kesehatan Lingkungan
                                            UKME-3. Gizi, Kesehatan Ibu dan Kesehatan Anak
                                            UKME-4. Imunisasi
                                            UKME-5. Pengendalian Penyakit Menular
                                            UKME-6. Pengendalian Penyakit Tidak Menular
                                            UKME-7. Perawatan Kesehatan Masyarakat
                                            Penyakit Menular Potensi KLB
                                            Penyakit Potensi KLB Menurut Desa/Kel
                                            Promosi Kesehatan Data Kemitraan
                                            UKMP. Kesehatan Kerja
                                            Program
                                                                                                                                    Pemeriksaan IMS
                                                                                                                                    Laporan Kunjungan Ibu Hamil
                                                                                                                                    Kohort KIA
                                                                                                                                    PKPR
                                                                                                                                    Konseling HIV
                                                                                                                                    Rekapitulasi MTBS MTBM
                                                                                                                                    PTM
                                                                                                                                    Kirim Laporan
                                                                                        Laporan Tahunan
                                            Laporan Eliminasi 3E
                                                                                        Laporan Tahunan
                                            Laporan Tahunan LSD 1
                                                                                                                                    Laporan Tahunan LSD 2
                                                                                                                                    Laporan Tahunan LSD 3
                                                                                                                                                                                
                                                                            
                                
                            
                                                    
                                                               
                        
                            
	.broadcastBox::-webkit-scrollbar-track
    {
        -webkit-box-shadow: inset 0 0 6px rgba(0,0,0,0.3);
        background-color: #F5F5F5;
      border-radius: 5px
    }

    .broadcastBox::-webkit-scrollbar
    {
        width: 10px;
        background-color: #F5F5F5;
      border-radius: 5px
    }

    .broadcastBox::-webkit-scrollbar-thumb
    {
        background-color: black;
        border: 2px solid black;
      border-radius: 5px
    }

    .fasBroadcast{
      font-size: 25pt;
      padding-bottom: 10px;
      color: black;
      margin-right: 40px;
      margin-left: 40px;
    }

    .broadcastBox{
      width: 400px;
      height: 0px;
      border-radius: 10px;
      transition: 0.5s;
      position: absolute;
      overflow-y: scroll;
      padding: 0px;
      left: -360px;
      margin-top: 5px;
      background-color: #F4F4F4;
      -webkit-box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.2);
      -moz-box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.1);
      box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.1);
    }

    .fasBroadcast:hover {
      color: #d63031;
    }


    .displayBroadcast{
      position: relative;
    }

    .contBroadcast{
      position: absolute;
      top: 0;
      width: 100%;
      height: 100%;
      background-color: #F4F4F4;
    }

    .contBroadcast:empty{
      display: none;
    }

    .stickBroadcast{
      text-align: center;  
      display: block;
      font-size: 50pt;
      padding-top: 70px;
      padding-left: 80px
    }

    .stickBroadcast:hover{
      color: black;
    }

    .centBroadcast{
      text-align: center;
      display: block;
    }

    .secBroadcast{
      padding: 25px 10px;
      background-color: #F4F4F4;
      transition: 0.5s;
    }

    .profContBroadcast{
      padding-left: 15px;
    }

    .profileBroadcast{
      -webkit-clip-path: circle(50% at 50% 50%);
      clip-path: circle(50% at 50% 50%);
      width: 75px;
      float: left;
    }

    .txtBroadcast{
      vertical-align: top;
      font-size: 1.25rem;
      padding: 5px 10px 0px 115px;
    }

    .subBroadcast{
      font-size: 1rem;
      color: grey;
    }

    .newBroadcast{
      border-style: none none solid none;
      border-color: red;
    }

    .secBroadcast:hover{
      background-color: #BFBFBF;
    }

    .headerBroadcast{
      padding: 5px;
    }

    .isiContentBroadcast{
      padding: 10px;
    }

    .judulBroadcast{
      padding-bottom: 10px;
    }

    .dotBroadcast {
      height: 9px;
      width: 9px;
      background-color: #9D2D15;
      border-radius: 50%;
      display: inline-block;
    }

    .containerNotifBroadcast img {
      transition: transform 0.25s ease;
      cursor: zoom-in;
    }

    .zoomCheckNotif:checked ~ label > img {
      transform: scale(2);
      cursor: zoom-out;
    }


  



    var openBroadcast = &quot;true&quot;
    let numClicks = 0;
    function showHideBroadcast() {
        if (openBroadcast == &quot;false&quot;) {
          $('.broadcastBox').removeAttr(&quot;style&quot;);
          openBroadcast = &quot;true&quot;;
          return false
        } 

        numClicks++;
        if (numClicks === 1) {
          singleClickTimer = setTimeout(() => {
            numClicks = 0;
          }, 400);
        } else if (numClicks === 2) {
          clearTimeout(singleClickTimer);
          numClicks = 0;
          return false
        }

        $.ajax({
            url: &quot;https://demo.epuskesmas.id/notifikasi/getform&quot;,
            method: 'GET',
            dataType: &quot;json&quot;,
            data: {
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(data) {
                $(&quot;#isiNotifBroadcast&quot;).html(data.data)
                setTimeout(function (){
                      if(openBroadcast == &quot;true&quot;) {
                          $('.broadcastBox').css({
                              height: &quot;60vh&quot;
                          });
                          openBroadcast = &quot;false&quot;;
                      }
                }, 100);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                console.log(xhr.status);
                console.log(thrownError);
            }
        });

        
    }


    function setPilihanBroadcast() {
        var broadcastPilih = $('select[name=&quot;broadcastPilihan&quot;]').val()

        $.ajax({
            url: &quot;https://demo.epuskesmas.id/notifikasi/getform&quot;,
            method: 'GET',
            dataType: &quot;json&quot;,
            data: {
                pilihanBroadcast: broadcastPilih,
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(data) {
                $('#contBroadcast').html(&quot;&quot;)
                $('#contBroadcast').html('&lt;br/>&lt;br/>&lt;span class=&quot;loading&quot; style=&quot;background-color:#F4F4F4!important;padding-top: 30px;padding-left: 90%;&quot;>&amp;nbsp;&lt;/span>')
                setTimeout(function (){
                  $('#contBroadcast').html(data.data)
                }, 300);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                console.log(xhr.status);
                console.log(thrownError);
            }
        }); 
    }

    function bacaSelengkapnyaBroadcast(ids) {
      $('.broadcastBox').removeAttr(&quot;style&quot;);
      openBroadcast = &quot;true&quot;;

      $.blockUI({
        css: {
            border: 'none',
            padding: '15px',
            backgroundColor: '#000',
            '-webkit-border-radius': '10px',
            '-moz-border-radius': '10px',
            opacity: .5,
            fontSize: '8px',
            color: '#fff'
            },
            message: '&lt;h3>Memuat....&lt;/h3>'
        });

      $.unblockUI()

      $.ajax({
          url: &quot;https://demo.epuskesmas.id/notifikasi/show&quot;,
          method: 'GET',
          dataType: &quot;json&quot;,
          data: {
              ids: ids,
              _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
          },
          success: function(data) {
              $(&quot;#modal_notif_broadcast_notifikasi&quot;).modal({ backdrop: 'static'});
              $(&quot;#modal_notif_broadcast_notifikasi .modal-dialog&quot;).addClass(&quot;modal-lg&quot;);
              $('#modal_notif_broadcast_notifikasi .modal-title').html(data.tipe);
              $('#modal_notif_broadcast_notifikasi .modal-form').html(data.data);
              console.log(data.ressRead)
              if (data.ressRead == false) {
                $(&quot;#dotBroadcast&quot;).addClass('hidden')
              }
          },
          error: function (xhr, ajaxOptions, thrownError) {
              alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
              console.log(xhr.status);
              console.log(thrownError);
          }
      });   
    }

    function skemaAction() {
      let skema = $(&quot;#skema&quot;).val()
      $(&quot;#skemaAction&quot;).addClass('loading')
      var broadcastPilih = $('select[name=&quot;broadcastPilihan&quot;]').val()
      $.ajax({
          url: &quot;https://demo.epuskesmas.id/notifikasi/getformother&quot;,
          method: 'GET',
          dataType: &quot;json&quot;,
          data: {
              skema: skema,
              pilihanBroadcast: broadcastPilih,
              _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
          },
          success: function(data) {
            setTimeout(function (){
            $(&quot;#skemaAction&quot;).removeClass('loading')
            let totalSkema = Number(skema) + 1
              $(&quot;#skema&quot;).val(totalSkema)
              $(&quot;#isiDetailNotif&quot;).append(data.data)

              if(data.nextAction != &quot;true&quot;) {
                $(&quot;#skemaAction&quot;).addClass(&quot;hidden&quot;)
              }
            }, 300);
          },
          error: function (xhr, ajaxOptions, thrownError) {
              $(&quot;#skemaAction&quot;).removeClass('loading')
              alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
              console.log(xhr.status);
              console.log(thrownError);
          }
      }); 
    }
                        
                        
                                                             
                                                                        
                                         Belum Bridging
                                        8      
                                    
                                                                                                                        
                                                    
                                
                                    Darel PUSKESMAS PUSKESMAS1 
                                
                                
                                    Profil
                                                                            Update
                                                                        Umpan Balik
                                                                            Bantuan
                                                                        
                                    
                                        
                                            Keluar
                                        

                                        
                                            
                                        
                                    
                                
                            
                                            
                
            
        

        
        
        
        

        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        

        
                    backgroundChart = &quot;images/gis/bg-grafik.png&quot;;
                
        
        

        
        
            


    Welcome
                    Dashboard Utama
                        Dashboard Antrian
                                        Dashboard Rawat Inap
                                    Dashboard SIP
                                    Dashboard KIA
            
        Tampilkan Data
            Tampilkan Data
                        Tampilkan Data
                
        
            
                ×
                Pilih Tanggal
                
                
                    
                    
                        
                            
                            
                        
                    
                    
                        Tampilkan
                    
                
                
                    
                
            

        
    
    


    
        
            
                
                
                                                                        PUSKESMAS PUSKESMAS1
                                                                PT. Infokes Indonesia Infokes e-Health Solutions
                
            
            
            
            
            
            
                                                                                        Selamat Datang di ePuskesmas NG!
                                                                
                    
                        Sistem Informasi Manajemen Pelayanan kesehatan
                        Berbasis Teknologi Informasi untuk Mewujudkan Pelayanan Prima 
                        disetiap unit kesehatan Indonesia (Puskesmas, Klinik, Praktek Dokter, Apotek, Posyandu).
                    
                
                                
            
        
        
            
                                                            
                       
                
                            
        
    


    
        
            
                
                     Pasien Data Lengkap -
                    
                        
                    
                    Data pasien terverifikasi lengkap
                
            
            
                
                     Pasien Sudah Dilayani -
                    
                        
                    
                    Pasien yang sudah dilayani di semua pelayanan
                
            
                        
                
                     Resep Obat Diberikan -
                    
                        
                    
                    Resep obat yang sudah diproses di apotek
                
            
                        
                
                     Pasien Sudah Membayar -
                    
                        
                    
                    Pasien Umum yang membayar di Kasir
                
            
            
                
                     Pasien BPJS Sudah Dilayani -
                    
                        
                    
                    Jumlah Pasien BPJS yang sudah dilayani
                
            
            
                
                     Pasien BPJS Sudah Bridging -
                    
                        
                    
                    Pasien BPJS yang sudah Bridging ke PCare
                
            
        
    
    
        
            
                
            
            
                
                

            
        
    


    
        
        
        Fullscreen
    
    
        
            Dashboard antrian realtime belum di setting
                                                                                                      
                            
                                KB
                                
                                    
                                        
                                            
                                                                                                                                                                                                 
                                                 
                                                 
                                                 
                                                 
                                                                                            
                                            
                                                
                                            
                                        
                                    
                                
                                  Total Antrian : 0 Pasien
                            
                        
                                                                                
    


    
        
        
        Fullscreen
    
    
        
            Dashboard rawat inap realtime belum di setting
                            
                    
                        
                            
                                Inap Umum
                                Mawar Hitam
                            
                        
                        
                        
                            
                                
                                    
                                        Tersedia : 
                                    
                                
                            
                            
                                
                                    
                                        Terisi : 
                                    
                                
                            
                        
                    
                
                            
                    
                        
                            
                                Inap Umum
                                bunga
                            
                        
                        
                        
                            
                                
                                    
                                        Tersedia : 
                                    
                                
                            
                            
                                
                                    
                                        Terisi : 
                                    
                                
                            
                        
                    
                
                            
                    
                        
                            
                                Inap Umum
                                Melati
                            
                        
                        
                        
                            
                                
                                    
                                        Tersedia : 
                                    
                                
                            
                            
                                
                                    
                                        Terisi : 
                                    
                                
                            
                        
                    
                
                            
                    
                        
                            
                                Inap Umum
                                Melati
                            
                        
                        
                        
                            
                                
                                    
                                        Tersedia : 
                                    
                                
                            
                            
                                
                                    
                                        Terisi : 
                                    
                                
                            
                        
                    
                
                    
    


            function openTab(evt, Name) {
            var i, tabcontent, tablinks;
            tabcontent = $(&quot;.tabcontent&quot;);
            for (i = 0; i &lt; tabcontent.length; i++) {
                tabcontent[i].style.display = &quot;none&quot;;
            }
            tablinks = $(&quot;.tablinks&quot;);
            for (i = 0; i &lt; tablinks.length; i++) {
                tablinks[i].className = tablinks[i].className.replace(&quot; active&quot;, &quot;&quot;);
            }
            document.getElementById(Name).style.display = &quot;block&quot;;

            $(&quot;button[data-id=tampilkan_data_utama]&quot;).hide();
            $(&quot;button[data-id=tampilkan_data_antrian]&quot;).hide();
            $(&quot;button[data-id=tampilkan_data_ranap]&quot;).hide();
            $(&quot;h4[id=statistik]&quot;).hide();
            if(Name == 'Dashboard'){
                $(&quot;button[data-id=tampilkan_data_utama]&quot;).show();
                $(&quot;h4[id=statistik]&quot;).show();
            }else if(Name == 'Antrian'){
                $(&quot;button[data-id=tampilkan_data_antrian]&quot;).show();
            }else if(Name == 'Ranap'){
                $(&quot;button[data-id=tampilkan_data_ranap]&quot;).show();
            }
        }
    
    function openUrl(url){
        window.open(url, '_self');
    }

    $(() => {
                    switch(window.location.hash){
                                case '#Antrian':
                    $('div[class=tab]').find(&quot;#btn_dashboard_antrian&quot;).click();
                break;
                                                case '#Ranap':
                    $('div[class=tab]').find(&quot;#btn_dashboard_ranap&quot;).click();
                break;
                                                case '#Dashboard':
                    $('div[class=tab]').find(&quot;#btn_dashboard_utama&quot;).click();
                break;
                                default:
                    $('div[class=tab]').find(&quot;#btn_dashboard_welcome&quot;).click();
            }
                $(&quot;div[id='Dashboard'] div[id*=box_] div&quot;).mouseover(function() {
            $(this).css('cursor', 'pointer');
        });
                                    showUpdateLog();
                                        setTimeout(function() {
                    showNotif();
                }, 2000);
                    
                                                           gagalBridgingCount();
                                            var cek = &quot;&quot;;
        if (cek) {
            var logo_ponpes = '/images/portal/logo/'+localStorage.getItem(&quot;img_logo_ponpes&quot;);
            $('img#logo-portal').attr('src', logo_ponpes);
        }
    });
    


    var date = new Date();
    var tanggal_now = new Date(date.getFullYear(), date.getMonth(), date.getDate()-1, 23, 59, 59, 59);
    $('.date-custom').datetimepicker({
        locale:'id',
        format:'DD-MM-YYYY',
        maxDate:tanggal_now,
        defaultDate: null,
        useCurrent: false,
        ignoreReadonly: true
    });

    $(`#showData`).bind('click', function() {
        $(`#jml_daftar, #jml_dilayani, #jml_resep, #jml_bayar, #jml_bpjs, #jml_bridging, #myChart, #myChart2, #myChart3`).addClass('loading');
        $('#box_daftar, #box_dilayani, #box_resep, #box_bayar, #box_bpjs, #box_bridging').removeAttr('ondblclick');
        loadPendaftaran();
        $(`#closeMyModal`).click();
    });

        if ('e-Puskesmas' == 'e-Bidan Personal') {
        var ctx = document.getElementById('myChart');
        var myChart = new Chart(ctx, {
            type: 'horizontalBar',
            data: {
                labels: [],
                datasets: []
            },
            options: {
                scales: {
                    yAxes: [{
                        ticks: {
                            beginAtZero: true
                        }
                    }]
                },
                title: {
                    display: true,
                    text: &quot;10 Penyakit Terbanyak Hari Ini&quot;,
                }
            }
        });
    }else{
        var ctx = document.getElementById('myChart');
        var myChart = new Chart(ctx, {
            type: 'bar',
            data: {
                labels: [],
                datasets: []
            },
            options: {
                scales: {
                    yAxes: [{
                        ticks: {
                            beginAtZero: true
                        }
                    }]
                },
                title: {
                    display: true,
                    text: &quot;Pasien Poli / Ruangan Hari Ini&quot;,
                }
            }
        });
    }

    var ctx2 = document.getElementById('myChart2');
    var myChart2 = new Chart(ctx2, {
        type: 'pie',
        data: {
            labels: ['Lama', 'Baru'],
            datasets: [{
                data: [],
                backgroundColor: ['#5555FF', '#55FF55'],
            }]
        },
        options: {
            legend: {
                display: true,
            },
            title: {
                display: true,
                text: &quot;Kunjungan Pasien Hari Ini&quot;,
            }
        }
    });
    var ctx3 = document.getElementById('myChart3');
    var myChart3 = new Chart(ctx3, {
        type: 'line',
        data: {
            labels: [],
            datasets: [{
                label: &quot;Jumlah Kunjungan&quot;,
                backgroundColor: &quot;rgba(38, 185, 154, 0.31)&quot;,
                borderColor: &quot;rgba(38, 185, 154, 0.7)&quot;,
                pointBorderColor: &quot;rgba(38, 185, 154, 0.7)&quot;,
                pointBackgroundColor: &quot;rgba(38, 185, 154, 0.7)&quot;,
                pointHoverBackgroundColor: &quot;#fff&quot;,
                pointHoverBorderColor: &quot;rgba(220,220,220,1)&quot;,
                data: [],
            }]
        },
        options: {
            legend: {
                display: false,
            },
            title: {
                display: true,
                text: &quot;Kunjungan 7 Hari Kebelakang&quot;,
            }
        }
    });
    
    function loadPendaftaran(callback) {
        $('#jml_daftar').addClass('loading');
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: 'true',
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: 'pendaftaran',
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                $('#statistik').html(&quot;Statistik : &quot; + $(&quot;#tanggal_custom&quot;).val());
                if(window.location.pathname == '/home'){
                    $('#box_daftar').attr('ondblclick', 'openUrl(&quot;https://demo.epuskesmas.id/pasien&quot;)');
                }
                $('#jml_daftar').html(result.pendaftaran.current+&quot;/&quot;+result.pendaftaran.count).removeClass('loading');
                $(&quot;#bar_daftar&quot;).html(result.pendaftaran.percent+&quot;%&quot;).css(&quot;width&quot;, result.pendaftaran.percent+&quot;%&quot;);
                loadPelayanan();
            }
        });
    }

    function loadPelayanan() {
        $('#jml_dilayani').addClass('loading');
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: 'true',
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: 'pelayanan',
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == '/home'){
                    $('#box_dilayani').attr('ondblclick', 'openUrl(&quot;https://demo.epuskesmas.id/pendaftaran?tanggal='+$(&quot;#tanggal_custom&quot;).val()+'&quot;)');
                }
                $('#jml_dilayani').html(result.pelayanan.current+&quot;/&quot;+result.pelayanan.count).removeClass('loading');
                $(&quot;#bar_dilayani&quot;).html(result.pelayanan.percent+&quot;%&quot;).css(&quot;width&quot;, result.pelayanan.percent+&quot;%&quot;);
                if ('e-Puskesmas' == 'e-Bidan Personal') {
                    loadObatPasien();
                } else {
                    loadResep();
                }
            }
        });
    }

    function loadResep() {
        $('#jml_resep').addClass('loading');
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: 'true',
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: 'resep',
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == '/home'){
                    $('#box_resep').attr('ondblclick', 'openUrl(&quot;https://demo.epuskesmas.id/resep?tanggal='+$(&quot;#tanggal_custom&quot;).val()+'&quot;)');
                }
                $('#jml_resep').html(result.resep.current+&quot;/&quot;+result.resep.count).removeClass('loading');
                $(&quot;#bar_resep&quot;).html(result.resep.percent+&quot;%&quot;).css(&quot;width&quot;, result.resep.percent+&quot;%&quot;);
                loadBayar();
            }
        });
    }

    function loadObatPasien() {
        $('#jml_obatpasien').addClass('loading');
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: 'true',
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: 'obatpasien',
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == '/home'){
                    $('#box_obatpasien').attr('ondblclick', 'openUrl(&quot;https://demo.epuskesmas.id/obatpasien?tanggal='+$(&quot;#tanggal_custom&quot;).val()+'&quot;)');
                }
                $('#jml_obatpasien').html(result.obatpasien.current+&quot;/&quot;+result.obatpasien.count).removeClass('loading');
                $(&quot;#bar_obatpasien&quot;).html(result.obatpasien.percent+&quot;%&quot;).css(&quot;width&quot;, result.obatpasien.percent+&quot;%&quot;);
                loadBayar();
            }
        });
    }

    function loadBayar() {
        $('#jml_bayar').addClass('loading');
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: 'true',
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: 'bayar',
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == '/home'){
                    $('#box_bayar').attr('ondblclick', 'openUrl(&quot;https://demo.epuskesmas.id/pembayaran?tanggal='+$(&quot;#tanggal_custom&quot;).val()+'&quot;)');
                }
                $('#jml_bayar').html(result.pembayaran.current+&quot;/&quot;+result.pembayaran.count).removeClass('loading');
                $(&quot;#bar_bayar&quot;).html(result.pembayaran.percent+&quot;%&quot;).css(&quot;width&quot;, result.pembayaran.percent+&quot;%&quot;);
                loadBpjs();
            }
        });
    }

    function loadBpjs() {
        $('#jml_bpjs').addClass('loading');
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: 'true',
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: 'bpjs',
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == '/home'){
                    $('#box_bpjs').attr('ondblclick', 'openUrl(&quot;https://demo.epuskesmas.id/pendaftaran?asuransi_id=0001&amp;tanggal='+$(&quot;#tanggal_custom&quot;).val()+'&quot;)');
                }
                $('#jml_bpjs').html(result.bpjs.current+&quot;/&quot;+result.bpjs.count).removeClass('loading');
                $(&quot;#bar_bpjs&quot;).html(result.bpjs.percent+&quot;%&quot;).css(&quot;width&quot;, result.bpjs.percent+&quot;%&quot;);
                loadBridging(result.bpjs.count);
            }
        });
    }

    function loadBridging(total) {
        $('#jml_bridging').addClass('loading');
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: 'true',
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: 'bridging',
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                total: total,
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == '/home'){
                    $('#box_bridging').attr('ondblclick', 'openUrl(&quot;https://demo.epuskesmas.id/sinkronbpjs?search[tanggal]='+$(&quot;#tanggal_custom&quot;).val()+'&quot;)');
                }
                $('#jml_bridging').html(result.bridgingbpjs.current+&quot;/&quot;+result.bridgingbpjs.count).removeClass('loading');
                $(&quot;#bar_bridging&quot;).html(result.bridgingbpjs.percent+&quot;%&quot;).css(&quot;width&quot;, result.bridgingbpjs.percent+&quot;%&quot;);
                loadDataBar();
            }
        });
    }

    function loadDataBar() {
        $(`#myChart`).addClass('loading');
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: 'true',
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: 'bar',
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                $(`#myChart`).removeClass('loading');
                myChart.data.datasets = result;
                if ('e-Puskesmas' == 'e-Bidan Personal') {
                    myChart.options.title.text = &quot;10 Penyakit Terbanyak &quot; + $(&quot;#tanggal_custom&quot;).val();
                } else {
                    myChart.options.title.text = &quot;Pasien Poli / Ruangan &quot; + $(&quot;#tanggal_custom&quot;).val();
                }
                myChart.update();
                loadDataPie();
            }
        });
    }

    function loadDataPie() {
        $(`#myChart2`).addClass('loading');
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: 'true',
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: 'pie',
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                $(`#myChart2`).removeClass('loading');
                myChart2.data.datasets[0].data = [result.lama, result.baru];
                myChart2.options.title.text = &quot;Kunjungan Pasien &quot; + $(&quot;#tanggal_custom&quot;).val();
                myChart2.update();
                loadDataLine();
            }
        });
    }

    function loadDataLine() {
        $(`#myChart3`).addClass('loading');
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: 'true',
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: 'line',
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                $(`#myChart3`).removeClass('loading');
                myChart3.data.labels = result.days;
                myChart3.data.datasets[0].data = result.data;
                myChart3.options.title.text = &quot;Kunjungan 7 Hari Kebelakang dari &quot; + $(&quot;#tanggal_custom&quot;).val();
                myChart3.update();
            }
        });
    }


        function loadDataAntrian() {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                antrian: 'true',
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(response) {
                if(response.ruangan.length){
                    response.ruangan.forEach(function(result, key) {
                        $(&quot;#P0000000001&quot;+result.id+&quot;1&quot;).html('&amp;nbsp;');
                        $(&quot;#P0000000001&quot;+result.id+&quot;2&quot;).html('&amp;nbsp;');
                        $(&quot;#P0000000001&quot;+result.id+&quot;3&quot;).html('&amp;nbsp;');
                        $(&quot;#P0000000001&quot;+result.id+&quot;4&quot;).html('&amp;nbsp;');
                        $(&quot;#P0000000001&quot;+result.id+&quot;5&quot;).html('&amp;nbsp;');
                        if(response.data.length){
                            if(response.data[key].length){
                                response.data[key].forEach(function(result2, key2) {
                                    let icon = '';
                                    switch (result2.pendaftaran.reg_type) {
                                        case 'mobile':
                                            icon = 'icon-mobile'
                                            break;
                                        case 'kiosk':
                                            icon = 'icon-kiosk'
                                            break;
                                        case 'sms':
                                            icon = 'icon-sms'
                                            break;
                                        default:
                                            icon = 'icon-puskesmas'
                                    }
                                    $(&quot;#P0000000001&quot;+result2.ruangan_id+(key2+1)).html('&lt;i class=&quot;epus-font '+icon+'&quot;>&lt;/i> &lt;strong>'+ result2.antrean +' - '+ result2.pendaftaran.pasien.nama.substring(0, 15) +'&lt;/strong>');
                                });
                            }
                            $(&quot;#P0000000001&quot;+result.id+&quot;TOTAL&quot;).html(response.count[key]);
                        }
                    });
                }
            }
        });
    }
            function loadDataRanap() {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                ranap: 'true',
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(response) {
                if(response.length > 0){
                    $.each(response, function (key, val) { 
                        let data = '';
                        $.each(val.bed_details, function (keyBed, valBed) { 
                            let txtColor = 'primary';
                            if (valBed.hasOwnProperty('pelayanan') &amp;&amp; valBed.pelayanan !== null 
                                &amp;&amp; valBed.pelayanan.hasOwnProperty('pendaftaran') &amp;&amp; valBed.pelayanan.pendaftaran !== null) {
                                txtColor = 'danger';
                            }
                            data += '&lt;i class=&quot;fa fa-bed text-'+ txtColor +'&quot; style=&quot;margin:3px;&quot; title=&quot;'+ valBed.nama +'&quot;>&lt;/i>';
                        });
                        $(&quot;#P0000000001&quot; + val.ruangan_id + val.id).html(data);
                        $(&quot;#P0000000001&quot; + val.ruangan_id + val.id + &quot;TERISI&quot;).html(val.terisi);
                        $(&quot;#P0000000001&quot; + val.ruangan_id + val.id + &quot;TERSEDIA&quot;).html(val.tersedia);
                    });
                }
            }
        });
    }
            
        
            
                 
            
        
        
        
        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                    
                
            
        

        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                        
                    
                
            
        

        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                    
                
            
        
        
            
                
                    
                        ×
                          Notifikasi
                    
                    
                        
                        
                    
                
            
        
                                                
	#modalBantuan {
        position: fixed;
        z-index: 99998; 
        height: 52px; 
        width: 52px;
        background-color: #25d366;
        text-align: center;
        bottom: 5%;
        left: 2%;
        border-radius: 50%;
        font-size: 40px;
    }

    #modalBantuan > i {
        color: white;
        padding-top: 5px;
    }






    function openBantuan(){
        Swal.fire({
            title: 'Apakah anda membutuhkan bantuan dari Team Customer Support?',
            type: 'info',
            showCancelButton: true,
            confirmButtonColor: '#3085d6',
            cancelButtonColor: '#d33',
            confirmButtonText: 'Ya',
            cancelButtonText: 'Tidak'
        }).then((result) => {
            if (result.value) {
                let notif_wa = &quot;https://api.whatsapp.com/send/?phone=6281277073300&amp;text=Hi,%20Saya%20pengguna%20ePuskesmas:%20,%20%20(darel)%20-%20P0000000001%20PUSKESMAS1%20-%20KABUPATEN%20BANDUNG%20memerlukan%20bantuan%20anda!&amp;app_absent=0&quot;
                window.open(notif_wa);
            }
        });
    }

    $(&quot;#modalBantuan&quot;).mousedown(function() {
        isDragging = false;
    }).mousemove(function() {
        isDragging = true;
    }).mouseup(function() {
        var wasDragging = isDragging;
        isDragging = false;
        if (!wasDragging) {
            openBantuan();
        }
    });

    dragElement(document.getElementById(&quot;modalBantuan&quot;));

    function dragElement(elmnt) {
      var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
      if (document.getElementById(elmnt.id + &quot;header&quot;)) {
        document.getElementById(elmnt.id + &quot;header&quot;).onmousedown = dragMouseDown;
      } else {
        elmnt.onmousedown = dragMouseDown;
      }

      function dragMouseDown(e) {
        e = e || window.event;
        e.preventDefault();
        pos3 = e.clientX;
        pos4 = e.clientY;
        document.onmouseup = closeDragElement;
        document.onmousemove = elementDrag;
      }

      function elementDrag(e) {
        e = e || window.event;
        e.preventDefault();
        pos1 = pos3 - e.clientX;
        pos2 = pos4 - e.clientY;
        pos3 = e.clientX;
        pos4 = e.clientY;
        elmnt.style.top = (elmnt.offsetTop - pos2) + &quot;px&quot;;
        elmnt.style.left = (elmnt.offsetLeft - pos1) + &quot;px&quot;;
      }

      function closeDragElement() {
        document.onmouseup = null;
        document.onmousemove = null;
      }
    }

    var modalBantuanDrag = document.getElementById('modalBantuan');
    modalBantuanDrag.addEventListener('touchmove', function(event) {
      event.preventDefault();
      if (event.targetTouches.length == 1) {
        var touch = event.targetTouches[0];
        modalBantuanDrag.style.left = touch.pageX + 'px';
        modalBantuanDrag.style.top = touch.pageY + 'px';
      }
    }, {passive: true});
                
            $(document).ready(function(){
                $(window).scroll(function () {
                    $('#icon_back').attr('style','margin-top:'+(parseInt(innerHeight)-150)+'px;margin-left:'+(parseInt(innerWidth)-70)+'px;z-index:1000;');
                    if ($(this).scrollTop() > 125) {
                        $('#top-link-block').removeClass('hidden');
                        $('#top-link-block').fadeIn();
                    }else{
                        $('#top-link-block').fadeOut();
                    }                    
                });

                let message_skrining= 'null';
                if(message_skrining == &quot;error_message&quot;){

                    let text_message = 'null';
                    let text_status = 'null';
                    if(text_status == 'null'){
                        text_status = 'error';
                    }
                    if(text_message != 'null'){
                        Swal.fire(
                          text_message,
                          '',
                          text_status
                        )
                    }
                }

                $('[data-toggle=&quot;tooltip&quot;]').tooltip();


                                                                                        let stt= localStorage.getItem(&quot;count_notif&quot;);
                            if(stt !== &quot;undefined&quot; &amp;&amp; stt !== &quot;0&quot; &amp;&amp; stt !== &quot;null&quot; ){
                                $(&quot;#kolom_gagal_bridging&quot;).html(stt);   
                            }
                                                                            
            });
            
            function aksipesan(){
                Swal.fire({
                    text: &quot;Apakah Anda ingin tutup pesan Webservice bpjs dan tidak di tampilkan lagi ?&quot;,
                    type: 'warning',
                    showCancelButton: true,
                    confirmButtonColor: '#3085d6',
                    cancelButtonColor: '#d33',
                    confirmButtonText: 'Ya',
                    cancelButtonText: 'Tutup Sekarang Saja'
                }).then((result) => {
                    if(result.value){
                        setSessionWebSocket(1, &quot;indikatorbpjs&quot;);
                        $('#notif-connection-bpjs').hide();
                    }

                    if (result.dismiss == &quot;cancel&quot;) {
                        $('#notif-connection-bpjs').hide();
                    }
                });
            }

                        function callAntreanKiosk(nomorAntrean, ruangan_code){
                                return false;
            }
            
            function deletePasienKiosk(pasien_id){
                                return false;
            }

            function deletePendaftaranKiosk(pendaftaran_id){
                                return false;
            }

            function syncPasienKiosk(){
                                return false;
            }

            function syncPendaftaranKiosk(){
                                return false;
            }
        
        
            var date = new Date();
            var tanggal = new Date(date.getFullYear(), date.getMonth(), date.getDate(), 23, 59, 59, 59);
            var bulan = new Date(date.getFullYear(), date.getMonth(), 1, 0, 0, 0);
            var tahun = new Date(date.getFullYear(), 1, 0, 0, 0);
                            var tanggal_min = new Date(date.getFullYear(), date.getMonth(), date.getDate(), 00, 00, 00, 00);
                        var sistole_diastole_anamnesa = &quot;&quot;;
            var default_sistole = &quot;&quot;;
            var default_diastole = &quot;&quot;;
        
        
                    
            
                
            function showUpdateLog(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/updatelog/showupdatelog&quot;,
                    method: 'GET',
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                    },
                    success: function(data) {
                        if(data.title != '' &amp;&amp; data.title != undefined){
                            $('#modal .modal-title').html(data.title);
                            $('#modal .modal-form').html(data.form);
                            openmodal(&quot;modal-lg&quot;);
                        }else{
                            if(from_menu){
                                alert(&quot;Belum ada update terbaru saat ini!&quot;,&quot;info&quot;);
                            }
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function showNotif(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/mpuskesmas/shownotif&quot;,
                    method: 'GET',
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                    },
                    success: function(data) {
                        if(data.title != '' &amp;&amp; data.title != undefined){
                            $('#modal_notif .modal-title').html(data.title);
                            $('#modal_notif .modal-form').html(data.form);
                            openmodal_notif(&quot;modal-lg&quot;);

                            if (data.notif=='NOTIF_02') {
                                $('#btn_close_suspend').hide();
                            }

                        }else{
                            if(from_menu){
                                alert(&quot;Belum ada update terbaru saat ini!&quot;,&quot;info&quot;);
                            }
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function showNotifBridging(from_menu = false)
            {
                $.blockUI({
                css: {
                    border: 'none',
                    padding: '15px',
                    backgroundColor: '#000',
                    '-webkit-border-radius': '10px',
                    '-moz-border-radius': '10px',
                    opacity: .5,
                    fontSize: '8px',
                    color: '#fff'
                    },
                    message: '&lt;h3>Memuat....&lt;/h3>'
                });

                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/home/shownotifbridging&quot;,
                    method: 'GET',
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                    },
                    success: function(data) {
                        $('#modal_notif_bridging .modal-title').html(data.title);
                        $('#modal_notif_bridging .modal-form').html(data.form);
                        openmodal_notif_bridging(&quot;modal-lg&quot;);
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function gagalBridgingCount(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/home/gagalbridgingcount&quot;,
                    method: 'GET',
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                    },
                    success: function(data) {
                        var x = document.getElementById(&quot;button_gagal_bridging&quot;);
                        if(data > 0){
                            $(&quot;#kolom_gagal_bridging&quot;).html(data);
                            localStorage.setItem('count_notif',data);    
                        }else{
                            localStorage.setItem('count_notif',&quot;0&quot;);
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

        function showRiwayatPelayanan(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayatpelayanan&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                id: data.id,
                puskesmas: data.puskesmas,
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;,
            },
            success: function(data) {
                $(&quot;#modal .modal-title&quot;).html(data.title);
                $(&quot;#modal .modal-form&quot;).html(data.form);
                openmodal(&quot;modal-lg&quot;);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                    console.log(xhr.status);
                    console.log(thrownError);
                }
            });
        }

        function showRiwayatKunjunganBpjs(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayatkunjunganbpjs&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                    id: data.id,
                    _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;,
                },
                success: function(data) {
                    $(&quot;#modal .modal-title&quot;).html(data.title);
                    $(&quot;#modal .modal-form&quot;).html(data.form);
                    openmodal(&quot;modal-lg&quot;);
                },
                error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                }
            });
        }

        function getRiwayatKunjunganBpjs(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/getriwayatkunjunganbpjs&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                    noasuransi: data.noasuransi,
                    _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;,
                },
                success: function(data) {
                    alert(data.message, data.status);
                    $(obj).removeClass('loading');
                    if(data.status == &quot;success&quot;){
                        window.location.reload(true);
                    }
                },
                error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                    console.log(xhr.status);
                    console.log(thrownError);
                }
            });
        }

        function setSessionWebSocket(action,to = null)
        {
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/home/setsessionwebsocket&quot;,
                method: 'POST',
                dataType: &quot;json&quot;,
                data: {
                    action: action,
                    for_customsession:to,
                    _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                },
                success: function(data) {
                    console.log(&quot;success&quot;);
                },
                error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                }
            });
        }

        
        

        
                        
                    $(`#checkEnvConfig`).html('&lt;center>Dashboard antrian realtime belum di setting&lt;/center>');
                    $(`#checkEnvConfigRanap`).html('&lt;center>Dashboard rawat inap realtime belum di setting&lt;/center>');
                
                        

/html[1]/body[1]</value>
      <webElementGuid>f29e0486-eeeb-4059-8468-c413dff47e25</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>9fb71220-19ec-4579-9279-5f6ac69cc13a</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>cdef31c6-7e06-4433-8374-cb25141e1e4f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
                            
	var cloud_host = &quot;&quot;;
									cloud_host = &quot; , &quot;'&quot; , &quot;http://10.10.10.10/home&quot; , &quot;'&quot; , &quot;;
				localStorage.setItem(&quot;epuskesmas-cloud-server&quot;, cloud_host);

	
	    #connection {
	        position: fixed;
	        font-weight: bold;
	        bottom: 5px;
	        left: 5px;
	        z-index: 99;
	        font-size: 16px;
	        border: none;
	        outline: none;
	        color: white;
	        cursor: pointer;
	        padding: 3px;
	        border-radius: 4px;
	    }
	    .online {
	      background: green;
	    }
	    .offline {
	      background: red;
	    }
	
	
	
	    window.onbeforeunload = function(event){
	    	var connection = document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).innerHTML;
	    	localStorage.setItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;, connection);
	    }
	    if (&quot; , &quot;'&quot; , &quot;serviceWorker&quot; , &quot;'&quot; , &quot; in navigator) {
	    	document.onreadystatechange = function(e){
	    		if(localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;) != null &amp;&amp; localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).length > 0){
		            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).innerHTML = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;);
		            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).toLowerCase();
	    		}
			}

	        window.addEventListener(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function () {
	            navigator.serviceWorker.register(&quot; , &quot;'&quot; , &quot;/service-worker.js&quot; , &quot;'&quot; , &quot;).then(function(worker){
		    		if(localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;) != null &amp;&amp; localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).length > 0){
		    			var loadConnection = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).toLowerCase();
									                if(nextConnection == &quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;){
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;);
			                }else{
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;);
			                }
								    			sweetAlert(localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).toLowerCase());
		    		}
	            });
	        });
	        
	        function send_message_to_service_worker(messages){
	            return new Promise(function(resolve, reject){
	                var messageChannel = new MessageChannel();
	                messageChannel.port1.onmessage = function(event){
	                    if(event.data.error){
	                        reject(event.data.error);
	                    }else{
	                        resolve(event.data);
	                    }
	                };
	                navigator.serviceWorker.controller.postMessage(messages, [messageChannel.port2]);
	            });
	        }

			navigator.serviceWorker.getRegistrations().then(registrations => {
				registrations.forEach(registration => {
					registration.unregister()
				})
			})

	        navigator.serviceWorker.addEventListener(&quot; , &quot;'&quot; , &quot;message&quot; , &quot;'&quot; , &quot;, function(event){
	        	if(event.data.type == &quot; , &quot;'&quot; , &quot;check connection&quot; , &quot;'&quot; , &quot;){
		            var previousConnection = document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className;
		            if(event.data.status != &quot;&quot;){
			            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).innerHTML = event.data.status;
			            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className = event.data.status.toLowerCase();
		            }
		            var nextConnection = document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className;
								            if(previousConnection == &quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;){
			                if(nextConnection == &quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;){
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;);
			                }else{
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;);
			                }
			            }else{
			                if(previousConnection == &quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;){
			                	if(nextConnection == &quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;){
				            		sweetAlert(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;);
			                	}
			                }
			            }
						        	}
	        });

	        function sweetAlert(message = null, timer = 3000){
						        	switch(message) {
					  	case &quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;:
		                	Swal.fire({
		                       	html: &quot; , &quot;'&quot; , &quot;&lt;h5>ePuskesmas NG mendeteksi adanya gangguan pada koneksi internet.&lt;br>Pastikan perangkat anda terhubung dengan koneksi internet.&lt;h5>&quot; , &quot;'&quot; , &quot;,
								imageUrl: &quot; , &quot;'&quot; , &quot;https://demo.epuskesmas.id/images/o2o/offline.png&quot; , &quot;'&quot; , &quot;,
								imageWidth: 400,
								imageHeight: 175,
								allowOutsideClick: false,
								showConfirmButton: &quot;1&quot;,
		                       	confirmButtonColor: &quot; , &quot;'&quot; , &quot;#224d09&quot; , &quot;'&quot; , &quot;,
		                       	confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya, Alihkan ke ePuskesmas NG Offline!&quot; , &quot;'&quot; , &quot;
		                   	}).then((result) => {
		                       	if (result.value) {
		                        	if(cloud_host == localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-cloud-server&quot; , &quot;'&quot; , &quot;)){
		                      			window.location = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-cloud-server&quot; , &quot;'&quot; , &quot;);
		                        	}
		                       	}
		                   	})
					    	break;
		        		default:
			  				Swal.fire({
		                        html: &quot; , &quot;'&quot; , &quot;&lt;h5>ePuskesmas NG telah terhubung (online).&lt;h5>&quot; , &quot;'&quot; , &quot;,
								imageUrl: &quot; , &quot;'&quot; , &quot;https://demo.epuskesmas.id/images/o2o/online.png&quot; , &quot;'&quot; , &quot;,
								imageWidth: 400,
								imageHeight: 175,
		                        timer: timer,
								showConfirmButton: false
		                    });
					}
					        }

	        setInterval(function(){
	            send_message_to_service_worker({&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;check connection&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;url&quot; , &quot;'&quot; , &quot; :&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;});
	        }, 8000);
	    }
	
                
        
            
                
                
                    
                        Toggle navigation
                        
                    
                
                
                                                                   
                                                                                                
                                        Pendaftaran 
                                        
                                                                                        Pasien &amp; KK
                                                                                                                                    Pendaftaran Pasien
                                                                                                                                    Rekam Medis
                                                                                                                                    Antrean
                                                                                         
                                            Panggil Antrean
                                                                                    
                                    
                                
                                                                    
                                        Pelayanan 
                                        
                                                                                            Pelayanan
                                                                                                Medis
                                                                                                                                                Resep
                                                                                                                                                Obat Pasien
                                                                                                                                                Penimbangan Balita
                                                                                                                                                Diare Advokasi
                                                                                                                                                Program Puskesmas
                                                                                                                                        
                                                                                            Pelayanan Luar Gedung
                                                                                                    Skrining Anak Sekolah
                                                                                                                                                   Skrining Santri
                                                                                                                                                    Skrining PTM
                                                                                                                                                    Sinkronisasi Kunjungan Sehat BPJS
                                                                                                                                                    Sinkronisasi Data Sihepi
                                                                                            
                                                                                            Pemeriksaan
                                                                                                Laboratorium
                                                                                                                                            
                                                                                            Pasien Pulang
                                                                                                Rujukan External
                                                                                               
                                                                                                Pasien Meninggal
                                                                                                                                                Pembayaran
                                                                                                                                                Detail Transaksi
                                                                                            
                                                                                            Survey Kepuasan/Testimoni Pasien
                                                                                                Survey Kepuasan &amp; Testimoni Pasien
                                                                                                                                                Pengelolaan Survey Kepuasan &amp; Testimoni Pasien
                                                                                            
                                                                                            Lroa
                                                                                                Lroa
                                                                                                                                    
                                    
                                                            
                                                            
                                    Pengelolaan 
                                    
                                                                                    Gudang Farmasi
                                                                                        Pemesanan Obat
                                                                                                                                    Penerimaan Obat
                                                                                                                                    Stok Obat
                                                                                                                                    Stok Opname
                                                                                                                                    Distribusi Obat
                                                                                    
                                                                                    Data Master
                                                                                        Pegawai
                                                                                                                                    Pengguna
                                                                                                                                                                                Ruangan
                                                                                                                                    Ruangan Akses
                                                                                                                                    Ruangan Puskesmas
                                                                                                                                    Ruangan JAKESH
                                                                                                                                    Puskesmas
                                                                                                                                    Perlengkapan
                                                                                                                                    Sasaran Program
                                                                                                                                                                                            Sasaran Proyeksi
                                                                                                                                                                                    Jadwal Pelayanan
                                                                                                                                    Jadwal Shift
                                                                                                                                    Libur
                                                                                                                                    Data Tindakan
                                                                                                                                    Data Imunisasi
                                                                                                                                    Kamar &amp; Bed
                                                                                                                                    Sekolah
                                                                                                                                    Asuransi
                                                                                                                                    Laboratorium
                                                                                                                                    Paket Laboratorium
                                                                                                                                    Obat
                                                                                                                                    Konfigurasi Dashboard Antrian
                                                                                                                                    Konfigurasi Form
                                                                                                                                    Konfigurasi Laboratorium
                                                                                                                                    SMS Konfigurasi
                                                                                                                                    Konfigurasi Pembulatan
                                                                                                                                    Asupan Makanan
                                                                                                                                    Industri
                                                                                                                                    PBF
                                                                                                                                                                                Komponen Tarif
                                                                                                                                    Metode Pembayaran
                                                                                                                                                                                Diagnosa
                                                                                                                                    Template Print
                                                                                                                                                                      Data Dasar Puskesmas
                                        
                                                                                    BPJS
                                                                                        Pengaturan BPJS
                                                                                                                                    RS Provider BPJS
                                                                                                                                    Poli FKTL BPJS
                                                                                                                                    Dokter BPJS
                                                                                                                                    Tindakan BPJS
                                                                                                                                    Laboratorium BPJS
                                                                                                                                    Obat BPJS
                                                                                                                                    Ruangan BPJS
                                                                                                                                    Pengaturan Bridging BPJS
                                                                                                                                    Sinkron BPJS
                                                                                                                                    Sinkron BPJS Jejaring
                                                                                                                                
                                        

                                        
                                                                                    SISRUTE
                                                                                        Konfigurasi
                                                                                                                                    Tujuan Rujukan
                                                                                                                                    Kriteria Khusus
                                                                                       
                                                                                        SDM
                                                                                                                                    Ruang Perawatan
                                                                                                                                    Pelayanan
                                                                                                                                    Sarana
                                                                                                                                    Kelas Perawatan
                                                                                                                                    Alkes
                                                                                    
                                                                                    Master Wilayah
                                                                                        Wilayah Kerja
                                                                                                                                    Wilayah Kerja Posyandu
                                                                                    
                                                                                    SIPTM
                                                                                        Puskesmas
                                                                                                                                    Diagnosis
                                                                                                                                    Sinkron SIPTM
                                                                                    
                                        
                                                                                    Scan
                                                                                        Scan QRCode
                                                                                                                                                                        Antrian
                                                                                        Antrian
                                                                                                                                                                        Speaker
                                                                                        Speaker
                                                                                                                                                                
                                
                            
                                                            
                                    GIS 
                                    
                                                                                Pasien
                                                                                                                        Penyakit
                                                                            
                                
                                                                                        
                                    Laporan 
                                    
                                                                                    Grafik
                                            Grafik
                                                                                                                            Dashboard SIP
                                        
                                                                                    Monitoring
                                                                                                                                    Penggunaan Fitur
                                                                                    
                                                                                    Laporan Harian
                                                                                                                                    Pendapatan Tindakan
                                                                                                                                    Kunjungan Pasien
                                                                                                                                    Kunjungan Pasien BPJS
                                                                                                                                    Pelayanan Pasien
                                                                                                                                    Pemeriksaan Medis
                                                                                                                                    Pelayanan Resep
                                                                                                                                    Pengeluaran Obat Pasien
                                                                                                                                    Pengeluaran Alat Kesehatan
                                                                                                                                    Pelayanan Laboratorium
                                                                                                                                    Rujukan Internal
                                                                                                                                    Rujukan External
                                                                                                                                    
                                                                                                    Tindakan Dokter/Perawat
                                                                                            
                                                                                                                                    Pemeriksaan Laboratorium
                                                                                                                                    Kinerja Puskesmas
                                                                                                                                    Pelayanan Ruangan
                                                                                                                                    Penyakit
                                                                                                                                    Asuhan Keperawatan
                                                                                                                                    Kunjungan PTM
                                                                                                                                    PKPR
                                                                                                                                    Alat Kesehatan Terbanyak
                                                                                                                                    Tindakan Terbanyak
                                                                                                                                    Obat Kadaluarsa
                                                                                                                                    Laporan Pendapatan
                                                                                                                                    Laporan Pendapatan Pendaftaran 
                                             
                                                                                                                                                                                        Laporan Pendapatan Kasir
                                                                                                                                                                                    Laporan Transaksi Kasir
                                                                                                                                                                                Laporan Skrining COVID-19
                                                                                                                                    Laporan Pengeluaran Obat
                                                                                                                                    Laporan Stok Opname
                                                                                                                                        Laporan Jasa Tenaga Medis
                                                                                                                                        Laporan Hepatitis C
                                                                                                                                        Laporan Diare
                                                                                                                                        Laporan Hepatitis B
                                                                                                                                        Pendapatan Laboratorium
                                                                                    
                                                                                    Laporan Mingguan
                                                                                        Laporan PWS Penyakit
                                                                                    
                                                                                    Laporan Bulanan
                                                                                        Laporan
                                                                                                                                    Data Dasar
                                                                                                                                    Kirim Report
                                                                                                                                    Monitoring Laporan
                                                                                                                                    SP3 LB1
                                                                                                                                    SP3 LB2 (LPLPO)
                                                                                                                                    SP3 LB3
                                                                                                                                    SP3 LB4
                                                                                                                                    UKP-1. Pelayanan Puskesmas
                                                                                                                                    UKP-2. Kesakitan Umum
                                                                                                                                    UKP-3. Kesakitan Gigi dan Mulut
                                                                                                                                    UKP-4. Data Kesakitan Terbanyak
                                                                                                                                    UKP-5. Data Kematian di Puskesmas
                                                                                                                                    UKP-6. LPLPO
                                                                                                                                    UKME-1. Promosi Kesehatan
                                            UKME-2. Kesehatan Lingkungan
                                            UKME-3. Gizi, Kesehatan Ibu dan Kesehatan Anak
                                            UKME-4. Imunisasi
                                            UKME-5. Pengendalian Penyakit Menular
                                            UKME-6. Pengendalian Penyakit Tidak Menular
                                            UKME-7. Perawatan Kesehatan Masyarakat
                                            Penyakit Menular Potensi KLB
                                            Penyakit Potensi KLB Menurut Desa/Kel
                                            Promosi Kesehatan Data Kemitraan
                                            UKMP. Kesehatan Kerja
                                            Program
                                                                                                                                    Pemeriksaan IMS
                                                                                                                                    Laporan Kunjungan Ibu Hamil
                                                                                                                                    Kohort KIA
                                                                                                                                    PKPR
                                                                                                                                    Konseling HIV
                                                                                                                                    Rekapitulasi MTBS MTBM
                                                                                                                                    PTM
                                                                                                                                    Kirim Laporan
                                                                                        Laporan Tahunan
                                            Laporan Eliminasi 3E
                                                                                        Laporan Tahunan
                                            Laporan Tahunan LSD 1
                                                                                                                                    Laporan Tahunan LSD 2
                                                                                                                                    Laporan Tahunan LSD 3
                                                                                                                                                                                
                                                                            
                                
                            
                                                    
                                                               
                        
                            
	.broadcastBox::-webkit-scrollbar-track
    {
        -webkit-box-shadow: inset 0 0 6px rgba(0,0,0,0.3);
        background-color: #F5F5F5;
      border-radius: 5px
    }

    .broadcastBox::-webkit-scrollbar
    {
        width: 10px;
        background-color: #F5F5F5;
      border-radius: 5px
    }

    .broadcastBox::-webkit-scrollbar-thumb
    {
        background-color: black;
        border: 2px solid black;
      border-radius: 5px
    }

    .fasBroadcast{
      font-size: 25pt;
      padding-bottom: 10px;
      color: black;
      margin-right: 40px;
      margin-left: 40px;
    }

    .broadcastBox{
      width: 400px;
      height: 0px;
      border-radius: 10px;
      transition: 0.5s;
      position: absolute;
      overflow-y: scroll;
      padding: 0px;
      left: -360px;
      margin-top: 5px;
      background-color: #F4F4F4;
      -webkit-box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.2);
      -moz-box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.1);
      box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.1);
    }

    .fasBroadcast:hover {
      color: #d63031;
    }


    .displayBroadcast{
      position: relative;
    }

    .contBroadcast{
      position: absolute;
      top: 0;
      width: 100%;
      height: 100%;
      background-color: #F4F4F4;
    }

    .contBroadcast:empty{
      display: none;
    }

    .stickBroadcast{
      text-align: center;  
      display: block;
      font-size: 50pt;
      padding-top: 70px;
      padding-left: 80px
    }

    .stickBroadcast:hover{
      color: black;
    }

    .centBroadcast{
      text-align: center;
      display: block;
    }

    .secBroadcast{
      padding: 25px 10px;
      background-color: #F4F4F4;
      transition: 0.5s;
    }

    .profContBroadcast{
      padding-left: 15px;
    }

    .profileBroadcast{
      -webkit-clip-path: circle(50% at 50% 50%);
      clip-path: circle(50% at 50% 50%);
      width: 75px;
      float: left;
    }

    .txtBroadcast{
      vertical-align: top;
      font-size: 1.25rem;
      padding: 5px 10px 0px 115px;
    }

    .subBroadcast{
      font-size: 1rem;
      color: grey;
    }

    .newBroadcast{
      border-style: none none solid none;
      border-color: red;
    }

    .secBroadcast:hover{
      background-color: #BFBFBF;
    }

    .headerBroadcast{
      padding: 5px;
    }

    .isiContentBroadcast{
      padding: 10px;
    }

    .judulBroadcast{
      padding-bottom: 10px;
    }

    .dotBroadcast {
      height: 9px;
      width: 9px;
      background-color: #9D2D15;
      border-radius: 50%;
      display: inline-block;
    }

    .containerNotifBroadcast img {
      transition: transform 0.25s ease;
      cursor: zoom-in;
    }

    .zoomCheckNotif:checked ~ label > img {
      transform: scale(2);
      cursor: zoom-out;
    }


  



    var openBroadcast = &quot;true&quot;
    let numClicks = 0;
    function showHideBroadcast() {
        if (openBroadcast == &quot;false&quot;) {
          $(&quot; , &quot;'&quot; , &quot;.broadcastBox&quot; , &quot;'&quot; , &quot;).removeAttr(&quot;style&quot;);
          openBroadcast = &quot;true&quot;;
          return false
        } 

        numClicks++;
        if (numClicks === 1) {
          singleClickTimer = setTimeout(() => {
            numClicks = 0;
          }, 400);
        } else if (numClicks === 2) {
          clearTimeout(singleClickTimer);
          numClicks = 0;
          return false
        }

        $.ajax({
            url: &quot;https://demo.epuskesmas.id/notifikasi/getform&quot;,
            method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
            dataType: &quot;json&quot;,
            data: {
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(data) {
                $(&quot;#isiNotifBroadcast&quot;).html(data.data)
                setTimeout(function (){
                      if(openBroadcast == &quot;true&quot;) {
                          $(&quot; , &quot;'&quot; , &quot;.broadcastBox&quot; , &quot;'&quot; , &quot;).css({
                              height: &quot;60vh&quot;
                          });
                          openBroadcast = &quot;false&quot;;
                      }
                }, 100);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                console.log(xhr.status);
                console.log(thrownError);
            }
        });

        
    }


    function setPilihanBroadcast() {
        var broadcastPilih = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;broadcastPilihan&quot;]&quot; , &quot;'&quot; , &quot;).val()

        $.ajax({
            url: &quot;https://demo.epuskesmas.id/notifikasi/getform&quot;,
            method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
            dataType: &quot;json&quot;,
            data: {
                pilihanBroadcast: broadcastPilih,
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(data) {
                $(&quot; , &quot;'&quot; , &quot;#contBroadcast&quot; , &quot;'&quot; , &quot;).html(&quot;&quot;)
                $(&quot; , &quot;'&quot; , &quot;#contBroadcast&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&lt;br/>&lt;br/>&lt;span class=&quot;loading&quot; style=&quot;background-color:#F4F4F4!important;padding-top: 30px;padding-left: 90%;&quot;>&amp;nbsp;&lt;/span>&quot; , &quot;'&quot; , &quot;)
                setTimeout(function (){
                  $(&quot; , &quot;'&quot; , &quot;#contBroadcast&quot; , &quot;'&quot; , &quot;).html(data.data)
                }, 300);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                console.log(xhr.status);
                console.log(thrownError);
            }
        }); 
    }

    function bacaSelengkapnyaBroadcast(ids) {
      $(&quot; , &quot;'&quot; , &quot;.broadcastBox&quot; , &quot;'&quot; , &quot;).removeAttr(&quot;style&quot;);
      openBroadcast = &quot;true&quot;;

      $.blockUI({
        css: {
            border: &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;,
            padding: &quot; , &quot;'&quot; , &quot;15px&quot; , &quot;'&quot; , &quot;,
            backgroundColor: &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;,
            &quot; , &quot;'&quot; , &quot;-webkit-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
            &quot; , &quot;'&quot; , &quot;-moz-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
            opacity: .5,
            fontSize: &quot; , &quot;'&quot; , &quot;8px&quot; , &quot;'&quot; , &quot;,
            color: &quot; , &quot;'&quot; , &quot;#fff&quot; , &quot;'&quot; , &quot;
            },
            message: &quot; , &quot;'&quot; , &quot;&lt;h3>Memuat....&lt;/h3>&quot; , &quot;'&quot; , &quot;
        });

      $.unblockUI()

      $.ajax({
          url: &quot;https://demo.epuskesmas.id/notifikasi/show&quot;,
          method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
          dataType: &quot;json&quot;,
          data: {
              ids: ids,
              _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
          },
          success: function(data) {
              $(&quot;#modal_notif_broadcast_notifikasi&quot;).modal({ backdrop: &quot; , &quot;'&quot; , &quot;static&quot; , &quot;'&quot; , &quot;});
              $(&quot;#modal_notif_broadcast_notifikasi .modal-dialog&quot;).addClass(&quot;modal-lg&quot;);
              $(&quot; , &quot;'&quot; , &quot;#modal_notif_broadcast_notifikasi .modal-title&quot; , &quot;'&quot; , &quot;).html(data.tipe);
              $(&quot; , &quot;'&quot; , &quot;#modal_notif_broadcast_notifikasi .modal-form&quot; , &quot;'&quot; , &quot;).html(data.data);
              console.log(data.ressRead)
              if (data.ressRead == false) {
                $(&quot;#dotBroadcast&quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;)
              }
          },
          error: function (xhr, ajaxOptions, thrownError) {
              alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
              console.log(xhr.status);
              console.log(thrownError);
          }
      });   
    }

    function skemaAction() {
      let skema = $(&quot;#skema&quot;).val()
      $(&quot;#skemaAction&quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;)
      var broadcastPilih = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;broadcastPilihan&quot;]&quot; , &quot;'&quot; , &quot;).val()
      $.ajax({
          url: &quot;https://demo.epuskesmas.id/notifikasi/getformother&quot;,
          method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
          dataType: &quot;json&quot;,
          data: {
              skema: skema,
              pilihanBroadcast: broadcastPilih,
              _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
          },
          success: function(data) {
            setTimeout(function (){
            $(&quot;#skemaAction&quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;)
            let totalSkema = Number(skema) + 1
              $(&quot;#skema&quot;).val(totalSkema)
              $(&quot;#isiDetailNotif&quot;).append(data.data)

              if(data.nextAction != &quot;true&quot;) {
                $(&quot;#skemaAction&quot;).addClass(&quot;hidden&quot;)
              }
            }, 300);
          },
          error: function (xhr, ajaxOptions, thrownError) {
              $(&quot;#skemaAction&quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;)
              alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
              console.log(xhr.status);
              console.log(thrownError);
          }
      }); 
    }
                        
                        
                                                             
                                                                        
                                         Belum Bridging
                                        8      
                                    
                                                                                                                        
                                                    
                                
                                    Darel PUSKESMAS PUSKESMAS1 
                                
                                
                                    Profil
                                                                            Update
                                                                        Umpan Balik
                                                                            Bantuan
                                                                        
                                    
                                        
                                            Keluar
                                        

                                        
                                            
                                        
                                    
                                
                            
                                            
                
            
        

        
        
        
        

        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        

        
                    backgroundChart = &quot;images/gis/bg-grafik.png&quot;;
                
        
        

        
        
            


    Welcome
                    Dashboard Utama
                        Dashboard Antrian
                                        Dashboard Rawat Inap
                                    Dashboard SIP
                                    Dashboard KIA
            
        Tampilkan Data
            Tampilkan Data
                        Tampilkan Data
                
        
            
                ×
                Pilih Tanggal
                
                
                    
                    
                        
                            
                            
                        
                    
                    
                        Tampilkan
                    
                
                
                    
                
            

        
    
    


    
        
            
                
                
                                                                        PUSKESMAS PUSKESMAS1
                                                                PT. Infokes Indonesia Infokes e-Health Solutions
                
            
            
            
            
            
            
                                                                                        Selamat Datang di ePuskesmas NG!
                                                                
                    
                        Sistem Informasi Manajemen Pelayanan kesehatan
                        Berbasis Teknologi Informasi untuk Mewujudkan Pelayanan Prima 
                        disetiap unit kesehatan Indonesia (Puskesmas, Klinik, Praktek Dokter, Apotek, Posyandu).
                    
                
                                
            
        
        
            
                                                            
                       
                
                            
        
    


    
        
            
                
                     Pasien Data Lengkap -
                    
                        
                    
                    Data pasien terverifikasi lengkap
                
            
            
                
                     Pasien Sudah Dilayani -
                    
                        
                    
                    Pasien yang sudah dilayani di semua pelayanan
                
            
                        
                
                     Resep Obat Diberikan -
                    
                        
                    
                    Resep obat yang sudah diproses di apotek
                
            
                        
                
                     Pasien Sudah Membayar -
                    
                        
                    
                    Pasien Umum yang membayar di Kasir
                
            
            
                
                     Pasien BPJS Sudah Dilayani -
                    
                        
                    
                    Jumlah Pasien BPJS yang sudah dilayani
                
            
            
                
                     Pasien BPJS Sudah Bridging -
                    
                        
                    
                    Pasien BPJS yang sudah Bridging ke PCare
                
            
        
    
    
        
            
                
            
            
                
                

            
        
    


    
        
        
        Fullscreen
    
    
        
            Dashboard antrian realtime belum di setting
                                                                                                      
                            
                                KB
                                
                                    
                                        
                                            
                                                                                                                                                                                                 
                                                 
                                                 
                                                 
                                                 
                                                                                            
                                            
                                                
                                            
                                        
                                    
                                
                                  Total Antrian : 0 Pasien
                            
                        
                                                                                
    


    
        
        
        Fullscreen
    
    
        
            Dashboard rawat inap realtime belum di setting
                            
                    
                        
                            
                                Inap Umum
                                Mawar Hitam
                            
                        
                        
                        
                            
                                
                                    
                                        Tersedia : 
                                    
                                
                            
                            
                                
                                    
                                        Terisi : 
                                    
                                
                            
                        
                    
                
                            
                    
                        
                            
                                Inap Umum
                                bunga
                            
                        
                        
                        
                            
                                
                                    
                                        Tersedia : 
                                    
                                
                            
                            
                                
                                    
                                        Terisi : 
                                    
                                
                            
                        
                    
                
                            
                    
                        
                            
                                Inap Umum
                                Melati
                            
                        
                        
                        
                            
                                
                                    
                                        Tersedia : 
                                    
                                
                            
                            
                                
                                    
                                        Terisi : 
                                    
                                
                            
                        
                    
                
                            
                    
                        
                            
                                Inap Umum
                                Melati
                            
                        
                        
                        
                            
                                
                                    
                                        Tersedia : 
                                    
                                
                            
                            
                                
                                    
                                        Terisi : 
                                    
                                
                            
                        
                    
                
                    
    


            function openTab(evt, Name) {
            var i, tabcontent, tablinks;
            tabcontent = $(&quot;.tabcontent&quot;);
            for (i = 0; i &lt; tabcontent.length; i++) {
                tabcontent[i].style.display = &quot;none&quot;;
            }
            tablinks = $(&quot;.tablinks&quot;);
            for (i = 0; i &lt; tablinks.length; i++) {
                tablinks[i].className = tablinks[i].className.replace(&quot; active&quot;, &quot;&quot;);
            }
            document.getElementById(Name).style.display = &quot;block&quot;;

            $(&quot;button[data-id=tampilkan_data_utama]&quot;).hide();
            $(&quot;button[data-id=tampilkan_data_antrian]&quot;).hide();
            $(&quot;button[data-id=tampilkan_data_ranap]&quot;).hide();
            $(&quot;h4[id=statistik]&quot;).hide();
            if(Name == &quot; , &quot;'&quot; , &quot;Dashboard&quot; , &quot;'&quot; , &quot;){
                $(&quot;button[data-id=tampilkan_data_utama]&quot;).show();
                $(&quot;h4[id=statistik]&quot;).show();
            }else if(Name == &quot; , &quot;'&quot; , &quot;Antrian&quot; , &quot;'&quot; , &quot;){
                $(&quot;button[data-id=tampilkan_data_antrian]&quot;).show();
            }else if(Name == &quot; , &quot;'&quot; , &quot;Ranap&quot; , &quot;'&quot; , &quot;){
                $(&quot;button[data-id=tampilkan_data_ranap]&quot;).show();
            }
        }
    
    function openUrl(url){
        window.open(url, &quot; , &quot;'&quot; , &quot;_self&quot; , &quot;'&quot; , &quot;);
    }

    $(() => {
                    switch(window.location.hash){
                                case &quot; , &quot;'&quot; , &quot;#Antrian&quot; , &quot;'&quot; , &quot;:
                    $(&quot; , &quot;'&quot; , &quot;div[class=tab]&quot; , &quot;'&quot; , &quot;).find(&quot;#btn_dashboard_antrian&quot;).click();
                break;
                                                case &quot; , &quot;'&quot; , &quot;#Ranap&quot; , &quot;'&quot; , &quot;:
                    $(&quot; , &quot;'&quot; , &quot;div[class=tab]&quot; , &quot;'&quot; , &quot;).find(&quot;#btn_dashboard_ranap&quot;).click();
                break;
                                                case &quot; , &quot;'&quot; , &quot;#Dashboard&quot; , &quot;'&quot; , &quot;:
                    $(&quot; , &quot;'&quot; , &quot;div[class=tab]&quot; , &quot;'&quot; , &quot;).find(&quot;#btn_dashboard_utama&quot;).click();
                break;
                                default:
                    $(&quot; , &quot;'&quot; , &quot;div[class=tab]&quot; , &quot;'&quot; , &quot;).find(&quot;#btn_dashboard_welcome&quot;).click();
            }
                $(&quot;div[id=&quot; , &quot;'&quot; , &quot;Dashboard&quot; , &quot;'&quot; , &quot;] div[id*=box_] div&quot;).mouseover(function() {
            $(this).css(&quot; , &quot;'&quot; , &quot;cursor&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;pointer&quot; , &quot;'&quot; , &quot;);
        });
                                    showUpdateLog();
                                        setTimeout(function() {
                    showNotif();
                }, 2000);
                    
                                                           gagalBridgingCount();
                                            var cek = &quot;&quot;;
        if (cek) {
            var logo_ponpes = &quot; , &quot;'&quot; , &quot;/images/portal/logo/&quot; , &quot;'&quot; , &quot;+localStorage.getItem(&quot;img_logo_ponpes&quot;);
            $(&quot; , &quot;'&quot; , &quot;img#logo-portal&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;src&quot; , &quot;'&quot; , &quot;, logo_ponpes);
        }
    });
    


    var date = new Date();
    var tanggal_now = new Date(date.getFullYear(), date.getMonth(), date.getDate()-1, 23, 59, 59, 59);
    $(&quot; , &quot;'&quot; , &quot;.date-custom&quot; , &quot;'&quot; , &quot;).datetimepicker({
        locale:&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;,
        format:&quot; , &quot;'&quot; , &quot;DD-MM-YYYY&quot; , &quot;'&quot; , &quot;,
        maxDate:tanggal_now,
        defaultDate: null,
        useCurrent: false,
        ignoreReadonly: true
    });

    $(`#showData`).bind(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function() {
        $(`#jml_daftar, #jml_dilayani, #jml_resep, #jml_bayar, #jml_bpjs, #jml_bridging, #myChart, #myChart2, #myChart3`).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;#box_daftar, #box_dilayani, #box_resep, #box_bayar, #box_bpjs, #box_bridging&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;);
        loadPendaftaran();
        $(`#closeMyModal`).click();
    });

        if (&quot; , &quot;'&quot; , &quot;e-Puskesmas&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;e-Bidan Personal&quot; , &quot;'&quot; , &quot;) {
        var ctx = document.getElementById(&quot; , &quot;'&quot; , &quot;myChart&quot; , &quot;'&quot; , &quot;);
        var myChart = new Chart(ctx, {
            type: &quot; , &quot;'&quot; , &quot;horizontalBar&quot; , &quot;'&quot; , &quot;,
            data: {
                labels: [],
                datasets: []
            },
            options: {
                scales: {
                    yAxes: [{
                        ticks: {
                            beginAtZero: true
                        }
                    }]
                },
                title: {
                    display: true,
                    text: &quot;10 Penyakit Terbanyak Hari Ini&quot;,
                }
            }
        });
    }else{
        var ctx = document.getElementById(&quot; , &quot;'&quot; , &quot;myChart&quot; , &quot;'&quot; , &quot;);
        var myChart = new Chart(ctx, {
            type: &quot; , &quot;'&quot; , &quot;bar&quot; , &quot;'&quot; , &quot;,
            data: {
                labels: [],
                datasets: []
            },
            options: {
                scales: {
                    yAxes: [{
                        ticks: {
                            beginAtZero: true
                        }
                    }]
                },
                title: {
                    display: true,
                    text: &quot;Pasien Poli / Ruangan Hari Ini&quot;,
                }
            }
        });
    }

    var ctx2 = document.getElementById(&quot; , &quot;'&quot; , &quot;myChart2&quot; , &quot;'&quot; , &quot;);
    var myChart2 = new Chart(ctx2, {
        type: &quot; , &quot;'&quot; , &quot;pie&quot; , &quot;'&quot; , &quot;,
        data: {
            labels: [&quot; , &quot;'&quot; , &quot;Lama&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Baru&quot; , &quot;'&quot; , &quot;],
            datasets: [{
                data: [],
                backgroundColor: [&quot; , &quot;'&quot; , &quot;#5555FF&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#55FF55&quot; , &quot;'&quot; , &quot;],
            }]
        },
        options: {
            legend: {
                display: true,
            },
            title: {
                display: true,
                text: &quot;Kunjungan Pasien Hari Ini&quot;,
            }
        }
    });
    var ctx3 = document.getElementById(&quot; , &quot;'&quot; , &quot;myChart3&quot; , &quot;'&quot; , &quot;);
    var myChart3 = new Chart(ctx3, {
        type: &quot; , &quot;'&quot; , &quot;line&quot; , &quot;'&quot; , &quot;,
        data: {
            labels: [],
            datasets: [{
                label: &quot;Jumlah Kunjungan&quot;,
                backgroundColor: &quot;rgba(38, 185, 154, 0.31)&quot;,
                borderColor: &quot;rgba(38, 185, 154, 0.7)&quot;,
                pointBorderColor: &quot;rgba(38, 185, 154, 0.7)&quot;,
                pointBackgroundColor: &quot;rgba(38, 185, 154, 0.7)&quot;,
                pointHoverBackgroundColor: &quot;#fff&quot;,
                pointHoverBorderColor: &quot;rgba(220,220,220,1)&quot;,
                data: [],
            }]
        },
        options: {
            legend: {
                display: false,
            },
            title: {
                display: true,
                text: &quot;Kunjungan 7 Hari Kebelakang&quot;,
            }
        }
    });
    
    function loadPendaftaran(callback) {
        $(&quot; , &quot;'&quot; , &quot;#jml_daftar&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;pendaftaran&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                $(&quot; , &quot;'&quot; , &quot;#statistik&quot; , &quot;'&quot; , &quot;).html(&quot;Statistik : &quot; + $(&quot;#tanggal_custom&quot;).val());
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_daftar&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/pasien&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_daftar&quot; , &quot;'&quot; , &quot;).html(result.pendaftaran.current+&quot;/&quot;+result.pendaftaran.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_daftar&quot;).html(result.pendaftaran.percent+&quot;%&quot;).css(&quot;width&quot;, result.pendaftaran.percent+&quot;%&quot;);
                loadPelayanan();
            }
        });
    }

    function loadPelayanan() {
        $(&quot; , &quot;'&quot; , &quot;#jml_dilayani&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;pelayanan&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_dilayani&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/pendaftaran?tanggal=&quot; , &quot;'&quot; , &quot;+$(&quot;#tanggal_custom&quot;).val()+&quot; , &quot;'&quot; , &quot;&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_dilayani&quot; , &quot;'&quot; , &quot;).html(result.pelayanan.current+&quot;/&quot;+result.pelayanan.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_dilayani&quot;).html(result.pelayanan.percent+&quot;%&quot;).css(&quot;width&quot;, result.pelayanan.percent+&quot;%&quot;);
                if (&quot; , &quot;'&quot; , &quot;e-Puskesmas&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;e-Bidan Personal&quot; , &quot;'&quot; , &quot;) {
                    loadObatPasien();
                } else {
                    loadResep();
                }
            }
        });
    }

    function loadResep() {
        $(&quot; , &quot;'&quot; , &quot;#jml_resep&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;resep&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_resep&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/resep?tanggal=&quot; , &quot;'&quot; , &quot;+$(&quot;#tanggal_custom&quot;).val()+&quot; , &quot;'&quot; , &quot;&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_resep&quot; , &quot;'&quot; , &quot;).html(result.resep.current+&quot;/&quot;+result.resep.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_resep&quot;).html(result.resep.percent+&quot;%&quot;).css(&quot;width&quot;, result.resep.percent+&quot;%&quot;);
                loadBayar();
            }
        });
    }

    function loadObatPasien() {
        $(&quot; , &quot;'&quot; , &quot;#jml_obatpasien&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;obatpasien&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_obatpasien&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/obatpasien?tanggal=&quot; , &quot;'&quot; , &quot;+$(&quot;#tanggal_custom&quot;).val()+&quot; , &quot;'&quot; , &quot;&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_obatpasien&quot; , &quot;'&quot; , &quot;).html(result.obatpasien.current+&quot;/&quot;+result.obatpasien.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_obatpasien&quot;).html(result.obatpasien.percent+&quot;%&quot;).css(&quot;width&quot;, result.obatpasien.percent+&quot;%&quot;);
                loadBayar();
            }
        });
    }

    function loadBayar() {
        $(&quot; , &quot;'&quot; , &quot;#jml_bayar&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;bayar&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_bayar&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/pembayaran?tanggal=&quot; , &quot;'&quot; , &quot;+$(&quot;#tanggal_custom&quot;).val()+&quot; , &quot;'&quot; , &quot;&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_bayar&quot; , &quot;'&quot; , &quot;).html(result.pembayaran.current+&quot;/&quot;+result.pembayaran.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_bayar&quot;).html(result.pembayaran.percent+&quot;%&quot;).css(&quot;width&quot;, result.pembayaran.percent+&quot;%&quot;);
                loadBpjs();
            }
        });
    }

    function loadBpjs() {
        $(&quot; , &quot;'&quot; , &quot;#jml_bpjs&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;bpjs&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_bpjs&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/pendaftaran?asuransi_id=0001&amp;tanggal=&quot; , &quot;'&quot; , &quot;+$(&quot;#tanggal_custom&quot;).val()+&quot; , &quot;'&quot; , &quot;&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_bpjs&quot; , &quot;'&quot; , &quot;).html(result.bpjs.current+&quot;/&quot;+result.bpjs.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_bpjs&quot;).html(result.bpjs.percent+&quot;%&quot;).css(&quot;width&quot;, result.bpjs.percent+&quot;%&quot;);
                loadBridging(result.bpjs.count);
            }
        });
    }

    function loadBridging(total) {
        $(&quot; , &quot;'&quot; , &quot;#jml_bridging&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;bridging&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                total: total,
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_bridging&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/sinkronbpjs?search[tanggal]=&quot; , &quot;'&quot; , &quot;+$(&quot;#tanggal_custom&quot;).val()+&quot; , &quot;'&quot; , &quot;&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_bridging&quot; , &quot;'&quot; , &quot;).html(result.bridgingbpjs.current+&quot;/&quot;+result.bridgingbpjs.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_bridging&quot;).html(result.bridgingbpjs.percent+&quot;%&quot;).css(&quot;width&quot;, result.bridgingbpjs.percent+&quot;%&quot;);
                loadDataBar();
            }
        });
    }

    function loadDataBar() {
        $(`#myChart`).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;bar&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                $(`#myChart`).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                myChart.data.datasets = result;
                if (&quot; , &quot;'&quot; , &quot;e-Puskesmas&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;e-Bidan Personal&quot; , &quot;'&quot; , &quot;) {
                    myChart.options.title.text = &quot;10 Penyakit Terbanyak &quot; + $(&quot;#tanggal_custom&quot;).val();
                } else {
                    myChart.options.title.text = &quot;Pasien Poli / Ruangan &quot; + $(&quot;#tanggal_custom&quot;).val();
                }
                myChart.update();
                loadDataPie();
            }
        });
    }

    function loadDataPie() {
        $(`#myChart2`).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;pie&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                $(`#myChart2`).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                myChart2.data.datasets[0].data = [result.lama, result.baru];
                myChart2.options.title.text = &quot;Kunjungan Pasien &quot; + $(&quot;#tanggal_custom&quot;).val();
                myChart2.update();
                loadDataLine();
            }
        });
    }

    function loadDataLine() {
        $(`#myChart3`).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;line&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                $(`#myChart3`).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                myChart3.data.labels = result.days;
                myChart3.data.datasets[0].data = result.data;
                myChart3.options.title.text = &quot;Kunjungan 7 Hari Kebelakang dari &quot; + $(&quot;#tanggal_custom&quot;).val();
                myChart3.update();
            }
        });
    }


        function loadDataAntrian() {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                antrian: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(response) {
                if(response.ruangan.length){
                    response.ruangan.forEach(function(result, key) {
                        $(&quot;#P0000000001&quot;+result.id+&quot;1&quot;).html(&quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot;);
                        $(&quot;#P0000000001&quot;+result.id+&quot;2&quot;).html(&quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot;);
                        $(&quot;#P0000000001&quot;+result.id+&quot;3&quot;).html(&quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot;);
                        $(&quot;#P0000000001&quot;+result.id+&quot;4&quot;).html(&quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot;);
                        $(&quot;#P0000000001&quot;+result.id+&quot;5&quot;).html(&quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot;);
                        if(response.data.length){
                            if(response.data[key].length){
                                response.data[key].forEach(function(result2, key2) {
                                    let icon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                                    switch (result2.pendaftaran.reg_type) {
                                        case &quot; , &quot;'&quot; , &quot;mobile&quot; , &quot;'&quot; , &quot;:
                                            icon = &quot; , &quot;'&quot; , &quot;icon-mobile&quot; , &quot;'&quot; , &quot;
                                            break;
                                        case &quot; , &quot;'&quot; , &quot;kiosk&quot; , &quot;'&quot; , &quot;:
                                            icon = &quot; , &quot;'&quot; , &quot;icon-kiosk&quot; , &quot;'&quot; , &quot;
                                            break;
                                        case &quot; , &quot;'&quot; , &quot;sms&quot; , &quot;'&quot; , &quot;:
                                            icon = &quot; , &quot;'&quot; , &quot;icon-sms&quot; , &quot;'&quot; , &quot;
                                            break;
                                        default:
                                            icon = &quot; , &quot;'&quot; , &quot;icon-puskesmas&quot; , &quot;'&quot; , &quot;
                                    }
                                    $(&quot;#P0000000001&quot;+result2.ruangan_id+(key2+1)).html(&quot; , &quot;'&quot; , &quot;&lt;i class=&quot;epus-font &quot; , &quot;'&quot; , &quot;+icon+&quot; , &quot;'&quot; , &quot;&quot;>&lt;/i> &lt;strong>&quot; , &quot;'&quot; , &quot;+ result2.antrean +&quot; , &quot;'&quot; , &quot; - &quot; , &quot;'&quot; , &quot;+ result2.pendaftaran.pasien.nama.substring(0, 15) +&quot; , &quot;'&quot; , &quot;&lt;/strong>&quot; , &quot;'&quot; , &quot;);
                                });
                            }
                            $(&quot;#P0000000001&quot;+result.id+&quot;TOTAL&quot;).html(response.count[key]);
                        }
                    });
                }
            }
        });
    }
            function loadDataRanap() {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                ranap: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(response) {
                if(response.length > 0){
                    $.each(response, function (key, val) { 
                        let data = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                        $.each(val.bed_details, function (keyBed, valBed) { 
                            let txtColor = &quot; , &quot;'&quot; , &quot;primary&quot; , &quot;'&quot; , &quot;;
                            if (valBed.hasOwnProperty(&quot; , &quot;'&quot; , &quot;pelayanan&quot; , &quot;'&quot; , &quot;) &amp;&amp; valBed.pelayanan !== null 
                                &amp;&amp; valBed.pelayanan.hasOwnProperty(&quot; , &quot;'&quot; , &quot;pendaftaran&quot; , &quot;'&quot; , &quot;) &amp;&amp; valBed.pelayanan.pendaftaran !== null) {
                                txtColor = &quot; , &quot;'&quot; , &quot;danger&quot; , &quot;'&quot; , &quot;;
                            }
                            data += &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-bed text-&quot; , &quot;'&quot; , &quot;+ txtColor +&quot; , &quot;'&quot; , &quot;&quot; style=&quot;margin:3px;&quot; title=&quot;&quot; , &quot;'&quot; , &quot;+ valBed.nama +&quot; , &quot;'&quot; , &quot;&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;;
                        });
                        $(&quot;#P0000000001&quot; + val.ruangan_id + val.id).html(data);
                        $(&quot;#P0000000001&quot; + val.ruangan_id + val.id + &quot;TERISI&quot;).html(val.terisi);
                        $(&quot;#P0000000001&quot; + val.ruangan_id + val.id + &quot;TERSEDIA&quot;).html(val.tersedia);
                    });
                }
            }
        });
    }
            
        
            
                 
            
        
        
        
        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                    
                
            
        

        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                        
                    
                
            
        

        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                    
                
            
        
        
            
                
                    
                        ×
                          Notifikasi
                    
                    
                        
                        
                    
                
            
        
                                                
	#modalBantuan {
        position: fixed;
        z-index: 99998; 
        height: 52px; 
        width: 52px;
        background-color: #25d366;
        text-align: center;
        bottom: 5%;
        left: 2%;
        border-radius: 50%;
        font-size: 40px;
    }

    #modalBantuan > i {
        color: white;
        padding-top: 5px;
    }






    function openBantuan(){
        Swal.fire({
            title: &quot; , &quot;'&quot; , &quot;Apakah anda membutuhkan bantuan dari Team Customer Support?&quot; , &quot;'&quot; , &quot;,
            type: &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;,
            showCancelButton: true,
            confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
            cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
            confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya&quot; , &quot;'&quot; , &quot;,
            cancelButtonText: &quot; , &quot;'&quot; , &quot;Tidak&quot; , &quot;'&quot; , &quot;
        }).then((result) => {
            if (result.value) {
                let notif_wa = &quot;https://api.whatsapp.com/send/?phone=6281277073300&amp;text=Hi,%20Saya%20pengguna%20ePuskesmas:%20,%20%20(darel)%20-%20P0000000001%20PUSKESMAS1%20-%20KABUPATEN%20BANDUNG%20memerlukan%20bantuan%20anda!&amp;app_absent=0&quot;
                window.open(notif_wa);
            }
        });
    }

    $(&quot;#modalBantuan&quot;).mousedown(function() {
        isDragging = false;
    }).mousemove(function() {
        isDragging = true;
    }).mouseup(function() {
        var wasDragging = isDragging;
        isDragging = false;
        if (!wasDragging) {
            openBantuan();
        }
    });

    dragElement(document.getElementById(&quot;modalBantuan&quot;));

    function dragElement(elmnt) {
      var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
      if (document.getElementById(elmnt.id + &quot;header&quot;)) {
        document.getElementById(elmnt.id + &quot;header&quot;).onmousedown = dragMouseDown;
      } else {
        elmnt.onmousedown = dragMouseDown;
      }

      function dragMouseDown(e) {
        e = e || window.event;
        e.preventDefault();
        pos3 = e.clientX;
        pos4 = e.clientY;
        document.onmouseup = closeDragElement;
        document.onmousemove = elementDrag;
      }

      function elementDrag(e) {
        e = e || window.event;
        e.preventDefault();
        pos1 = pos3 - e.clientX;
        pos2 = pos4 - e.clientY;
        pos3 = e.clientX;
        pos4 = e.clientY;
        elmnt.style.top = (elmnt.offsetTop - pos2) + &quot;px&quot;;
        elmnt.style.left = (elmnt.offsetLeft - pos1) + &quot;px&quot;;
      }

      function closeDragElement() {
        document.onmouseup = null;
        document.onmousemove = null;
      }
    }

    var modalBantuanDrag = document.getElementById(&quot; , &quot;'&quot; , &quot;modalBantuan&quot; , &quot;'&quot; , &quot;);
    modalBantuanDrag.addEventListener(&quot; , &quot;'&quot; , &quot;touchmove&quot; , &quot;'&quot; , &quot;, function(event) {
      event.preventDefault();
      if (event.targetTouches.length == 1) {
        var touch = event.targetTouches[0];
        modalBantuanDrag.style.left = touch.pageX + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;;
        modalBantuanDrag.style.top = touch.pageY + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;;
      }
    }, {passive: true});
                
            $(document).ready(function(){
                $(window).scroll(function () {
                    $(&quot; , &quot;'&quot; , &quot;#icon_back&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;margin-top:&quot; , &quot;'&quot; , &quot;+(parseInt(innerHeight)-150)+&quot; , &quot;'&quot; , &quot;px;margin-left:&quot; , &quot;'&quot; , &quot;+(parseInt(innerWidth)-70)+&quot; , &quot;'&quot; , &quot;px;z-index:1000;&quot; , &quot;'&quot; , &quot;);
                    if ($(this).scrollTop() > 125) {
                        $(&quot; , &quot;'&quot; , &quot;#top-link-block&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
                        $(&quot; , &quot;'&quot; , &quot;#top-link-block&quot; , &quot;'&quot; , &quot;).fadeIn();
                    }else{
                        $(&quot; , &quot;'&quot; , &quot;#top-link-block&quot; , &quot;'&quot; , &quot;).fadeOut();
                    }                    
                });

                let message_skrining= &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;;
                if(message_skrining == &quot;error_message&quot;){

                    let text_message = &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;;
                    let text_status = &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;;
                    if(text_status == &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;){
                        text_status = &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;;
                    }
                    if(text_message != &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;){
                        Swal.fire(
                          text_message,
                          &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
                          text_status
                        )
                    }
                }

                $(&quot; , &quot;'&quot; , &quot;[data-toggle=&quot;tooltip&quot;]&quot; , &quot;'&quot; , &quot;).tooltip();


                                                                                        let stt= localStorage.getItem(&quot;count_notif&quot;);
                            if(stt !== &quot;undefined&quot; &amp;&amp; stt !== &quot;0&quot; &amp;&amp; stt !== &quot;null&quot; ){
                                $(&quot;#kolom_gagal_bridging&quot;).html(stt);   
                            }
                                                                            
            });
            
            function aksipesan(){
                Swal.fire({
                    text: &quot;Apakah Anda ingin tutup pesan Webservice bpjs dan tidak di tampilkan lagi ?&quot;,
                    type: &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;,
                    showCancelButton: true,
                    confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
                    cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
                    confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya&quot; , &quot;'&quot; , &quot;,
                    cancelButtonText: &quot; , &quot;'&quot; , &quot;Tutup Sekarang Saja&quot; , &quot;'&quot; , &quot;
                }).then((result) => {
                    if(result.value){
                        setSessionWebSocket(1, &quot;indikatorbpjs&quot;);
                        $(&quot; , &quot;'&quot; , &quot;#notif-connection-bpjs&quot; , &quot;'&quot; , &quot;).hide();
                    }

                    if (result.dismiss == &quot;cancel&quot;) {
                        $(&quot; , &quot;'&quot; , &quot;#notif-connection-bpjs&quot; , &quot;'&quot; , &quot;).hide();
                    }
                });
            }

                        function callAntreanKiosk(nomorAntrean, ruangan_code){
                                return false;
            }
            
            function deletePasienKiosk(pasien_id){
                                return false;
            }

            function deletePendaftaranKiosk(pendaftaran_id){
                                return false;
            }

            function syncPasienKiosk(){
                                return false;
            }

            function syncPendaftaranKiosk(){
                                return false;
            }
        
        
            var date = new Date();
            var tanggal = new Date(date.getFullYear(), date.getMonth(), date.getDate(), 23, 59, 59, 59);
            var bulan = new Date(date.getFullYear(), date.getMonth(), 1, 0, 0, 0);
            var tahun = new Date(date.getFullYear(), 1, 0, 0, 0);
                            var tanggal_min = new Date(date.getFullYear(), date.getMonth(), date.getDate(), 00, 00, 00, 00);
                        var sistole_diastole_anamnesa = &quot;&quot;;
            var default_sistole = &quot;&quot;;
            var default_diastole = &quot;&quot;;
        
        
                    
            
                
            function showUpdateLog(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/updatelog/showupdatelog&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                    },
                    success: function(data) {
                        if(data.title != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; data.title != undefined){
                            $(&quot; , &quot;'&quot; , &quot;#modal .modal-title&quot; , &quot;'&quot; , &quot;).html(data.title);
                            $(&quot; , &quot;'&quot; , &quot;#modal .modal-form&quot; , &quot;'&quot; , &quot;).html(data.form);
                            openmodal(&quot;modal-lg&quot;);
                        }else{
                            if(from_menu){
                                alert(&quot;Belum ada update terbaru saat ini!&quot;,&quot;info&quot;);
                            }
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function showNotif(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/mpuskesmas/shownotif&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                    },
                    success: function(data) {
                        if(data.title != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; data.title != undefined){
                            $(&quot; , &quot;'&quot; , &quot;#modal_notif .modal-title&quot; , &quot;'&quot; , &quot;).html(data.title);
                            $(&quot; , &quot;'&quot; , &quot;#modal_notif .modal-form&quot; , &quot;'&quot; , &quot;).html(data.form);
                            openmodal_notif(&quot;modal-lg&quot;);

                            if (data.notif==&quot; , &quot;'&quot; , &quot;NOTIF_02&quot; , &quot;'&quot; , &quot;) {
                                $(&quot; , &quot;'&quot; , &quot;#btn_close_suspend&quot; , &quot;'&quot; , &quot;).hide();
                            }

                        }else{
                            if(from_menu){
                                alert(&quot;Belum ada update terbaru saat ini!&quot;,&quot;info&quot;);
                            }
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function showNotifBridging(from_menu = false)
            {
                $.blockUI({
                css: {
                    border: &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;,
                    padding: &quot; , &quot;'&quot; , &quot;15px&quot; , &quot;'&quot; , &quot;,
                    backgroundColor: &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;-webkit-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;-moz-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
                    opacity: .5,
                    fontSize: &quot; , &quot;'&quot; , &quot;8px&quot; , &quot;'&quot; , &quot;,
                    color: &quot; , &quot;'&quot; , &quot;#fff&quot; , &quot;'&quot; , &quot;
                    },
                    message: &quot; , &quot;'&quot; , &quot;&lt;h3>Memuat....&lt;/h3>&quot; , &quot;'&quot; , &quot;
                });

                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/home/shownotifbridging&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                    },
                    success: function(data) {
                        $(&quot; , &quot;'&quot; , &quot;#modal_notif_bridging .modal-title&quot; , &quot;'&quot; , &quot;).html(data.title);
                        $(&quot; , &quot;'&quot; , &quot;#modal_notif_bridging .modal-form&quot; , &quot;'&quot; , &quot;).html(data.form);
                        openmodal_notif_bridging(&quot;modal-lg&quot;);
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function gagalBridgingCount(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/home/gagalbridgingcount&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                    },
                    success: function(data) {
                        var x = document.getElementById(&quot;button_gagal_bridging&quot;);
                        if(data > 0){
                            $(&quot;#kolom_gagal_bridging&quot;).html(data);
                            localStorage.setItem(&quot; , &quot;'&quot; , &quot;count_notif&quot; , &quot;'&quot; , &quot;,data);    
                        }else{
                            localStorage.setItem(&quot; , &quot;'&quot; , &quot;count_notif&quot; , &quot;'&quot; , &quot;,&quot;0&quot;);
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

        function showRiwayatPelayanan(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayatpelayanan&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                id: data.id,
                puskesmas: data.puskesmas,
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;,
            },
            success: function(data) {
                $(&quot;#modal .modal-title&quot;).html(data.title);
                $(&quot;#modal .modal-form&quot;).html(data.form);
                openmodal(&quot;modal-lg&quot;);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                    console.log(xhr.status);
                    console.log(thrownError);
                }
            });
        }

        function showRiwayatKunjunganBpjs(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayatkunjunganbpjs&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                    id: data.id,
                    _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;,
                },
                success: function(data) {
                    $(&quot;#modal .modal-title&quot;).html(data.title);
                    $(&quot;#modal .modal-form&quot;).html(data.form);
                    openmodal(&quot;modal-lg&quot;);
                },
                error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                }
            });
        }

        function getRiwayatKunjunganBpjs(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/getriwayatkunjunganbpjs&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                    noasuransi: data.noasuransi,
                    _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;,
                },
                success: function(data) {
                    alert(data.message, data.status);
                    $(obj).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                    if(data.status == &quot;success&quot;){
                        window.location.reload(true);
                    }
                },
                error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                    console.log(xhr.status);
                    console.log(thrownError);
                }
            });
        }

        function setSessionWebSocket(action,to = null)
        {
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/home/setsessionwebsocket&quot;,
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                dataType: &quot;json&quot;,
                data: {
                    action: action,
                    for_customsession:to,
                    _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                },
                success: function(data) {
                    console.log(&quot;success&quot;);
                },
                error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                }
            });
        }

        
        

        
                        
                    $(`#checkEnvConfig`).html(&quot; , &quot;'&quot; , &quot;&lt;center>Dashboard antrian realtime belum di setting&lt;/center>&quot; , &quot;'&quot; , &quot;);
                    $(`#checkEnvConfigRanap`).html(&quot; , &quot;'&quot; , &quot;&lt;center>Dashboard rawat inap realtime belum di setting&lt;/center>&quot; , &quot;'&quot; , &quot;);
                
                        

/html[1]/body[1]&quot;) or . = concat(&quot;
                            
	var cloud_host = &quot;&quot;;
									cloud_host = &quot; , &quot;'&quot; , &quot;http://10.10.10.10/home&quot; , &quot;'&quot; , &quot;;
				localStorage.setItem(&quot;epuskesmas-cloud-server&quot;, cloud_host);

	
	    #connection {
	        position: fixed;
	        font-weight: bold;
	        bottom: 5px;
	        left: 5px;
	        z-index: 99;
	        font-size: 16px;
	        border: none;
	        outline: none;
	        color: white;
	        cursor: pointer;
	        padding: 3px;
	        border-radius: 4px;
	    }
	    .online {
	      background: green;
	    }
	    .offline {
	      background: red;
	    }
	
	
	
	    window.onbeforeunload = function(event){
	    	var connection = document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).innerHTML;
	    	localStorage.setItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;, connection);
	    }
	    if (&quot; , &quot;'&quot; , &quot;serviceWorker&quot; , &quot;'&quot; , &quot; in navigator) {
	    	document.onreadystatechange = function(e){
	    		if(localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;) != null &amp;&amp; localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).length > 0){
		            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).innerHTML = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;);
		            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).toLowerCase();
	    		}
			}

	        window.addEventListener(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function () {
	            navigator.serviceWorker.register(&quot; , &quot;'&quot; , &quot;/service-worker.js&quot; , &quot;'&quot; , &quot;).then(function(worker){
		    		if(localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;) != null &amp;&amp; localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).length > 0){
		    			var loadConnection = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).toLowerCase();
									                if(nextConnection == &quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;){
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;);
			                }else{
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;);
			                }
								    			sweetAlert(localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).toLowerCase());
		    		}
	            });
	        });
	        
	        function send_message_to_service_worker(messages){
	            return new Promise(function(resolve, reject){
	                var messageChannel = new MessageChannel();
	                messageChannel.port1.onmessage = function(event){
	                    if(event.data.error){
	                        reject(event.data.error);
	                    }else{
	                        resolve(event.data);
	                    }
	                };
	                navigator.serviceWorker.controller.postMessage(messages, [messageChannel.port2]);
	            });
	        }

			navigator.serviceWorker.getRegistrations().then(registrations => {
				registrations.forEach(registration => {
					registration.unregister()
				})
			})

	        navigator.serviceWorker.addEventListener(&quot; , &quot;'&quot; , &quot;message&quot; , &quot;'&quot; , &quot;, function(event){
	        	if(event.data.type == &quot; , &quot;'&quot; , &quot;check connection&quot; , &quot;'&quot; , &quot;){
		            var previousConnection = document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className;
		            if(event.data.status != &quot;&quot;){
			            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).innerHTML = event.data.status;
			            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className = event.data.status.toLowerCase();
		            }
		            var nextConnection = document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className;
								            if(previousConnection == &quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;){
			                if(nextConnection == &quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;){
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;);
			                }else{
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;);
			                }
			            }else{
			                if(previousConnection == &quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;){
			                	if(nextConnection == &quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;){
				            		sweetAlert(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;);
			                	}
			                }
			            }
						        	}
	        });

	        function sweetAlert(message = null, timer = 3000){
						        	switch(message) {
					  	case &quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;:
		                	Swal.fire({
		                       	html: &quot; , &quot;'&quot; , &quot;&lt;h5>ePuskesmas NG mendeteksi adanya gangguan pada koneksi internet.&lt;br>Pastikan perangkat anda terhubung dengan koneksi internet.&lt;h5>&quot; , &quot;'&quot; , &quot;,
								imageUrl: &quot; , &quot;'&quot; , &quot;https://demo.epuskesmas.id/images/o2o/offline.png&quot; , &quot;'&quot; , &quot;,
								imageWidth: 400,
								imageHeight: 175,
								allowOutsideClick: false,
								showConfirmButton: &quot;1&quot;,
		                       	confirmButtonColor: &quot; , &quot;'&quot; , &quot;#224d09&quot; , &quot;'&quot; , &quot;,
		                       	confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya, Alihkan ke ePuskesmas NG Offline!&quot; , &quot;'&quot; , &quot;
		                   	}).then((result) => {
		                       	if (result.value) {
		                        	if(cloud_host == localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-cloud-server&quot; , &quot;'&quot; , &quot;)){
		                      			window.location = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-cloud-server&quot; , &quot;'&quot; , &quot;);
		                        	}
		                       	}
		                   	})
					    	break;
		        		default:
			  				Swal.fire({
		                        html: &quot; , &quot;'&quot; , &quot;&lt;h5>ePuskesmas NG telah terhubung (online).&lt;h5>&quot; , &quot;'&quot; , &quot;,
								imageUrl: &quot; , &quot;'&quot; , &quot;https://demo.epuskesmas.id/images/o2o/online.png&quot; , &quot;'&quot; , &quot;,
								imageWidth: 400,
								imageHeight: 175,
		                        timer: timer,
								showConfirmButton: false
		                    });
					}
					        }

	        setInterval(function(){
	            send_message_to_service_worker({&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;check connection&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;url&quot; , &quot;'&quot; , &quot; :&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;});
	        }, 8000);
	    }
	
                
        
            
                
                
                    
                        Toggle navigation
                        
                    
                
                
                                                                   
                                                                                                
                                        Pendaftaran 
                                        
                                                                                        Pasien &amp; KK
                                                                                                                                    Pendaftaran Pasien
                                                                                                                                    Rekam Medis
                                                                                                                                    Antrean
                                                                                         
                                            Panggil Antrean
                                                                                    
                                    
                                
                                                                    
                                        Pelayanan 
                                        
                                                                                            Pelayanan
                                                                                                Medis
                                                                                                                                                Resep
                                                                                                                                                Obat Pasien
                                                                                                                                                Penimbangan Balita
                                                                                                                                                Diare Advokasi
                                                                                                                                                Program Puskesmas
                                                                                                                                        
                                                                                            Pelayanan Luar Gedung
                                                                                                    Skrining Anak Sekolah
                                                                                                                                                   Skrining Santri
                                                                                                                                                    Skrining PTM
                                                                                                                                                    Sinkronisasi Kunjungan Sehat BPJS
                                                                                                                                                    Sinkronisasi Data Sihepi
                                                                                            
                                                                                            Pemeriksaan
                                                                                                Laboratorium
                                                                                                                                            
                                                                                            Pasien Pulang
                                                                                                Rujukan External
                                                                                               
                                                                                                Pasien Meninggal
                                                                                                                                                Pembayaran
                                                                                                                                                Detail Transaksi
                                                                                            
                                                                                            Survey Kepuasan/Testimoni Pasien
                                                                                                Survey Kepuasan &amp; Testimoni Pasien
                                                                                                                                                Pengelolaan Survey Kepuasan &amp; Testimoni Pasien
                                                                                            
                                                                                            Lroa
                                                                                                Lroa
                                                                                                                                    
                                    
                                                            
                                                            
                                    Pengelolaan 
                                    
                                                                                    Gudang Farmasi
                                                                                        Pemesanan Obat
                                                                                                                                    Penerimaan Obat
                                                                                                                                    Stok Obat
                                                                                                                                    Stok Opname
                                                                                                                                    Distribusi Obat
                                                                                    
                                                                                    Data Master
                                                                                        Pegawai
                                                                                                                                    Pengguna
                                                                                                                                                                                Ruangan
                                                                                                                                    Ruangan Akses
                                                                                                                                    Ruangan Puskesmas
                                                                                                                                    Ruangan JAKESH
                                                                                                                                    Puskesmas
                                                                                                                                    Perlengkapan
                                                                                                                                    Sasaran Program
                                                                                                                                                                                            Sasaran Proyeksi
                                                                                                                                                                                    Jadwal Pelayanan
                                                                                                                                    Jadwal Shift
                                                                                                                                    Libur
                                                                                                                                    Data Tindakan
                                                                                                                                    Data Imunisasi
                                                                                                                                    Kamar &amp; Bed
                                                                                                                                    Sekolah
                                                                                                                                    Asuransi
                                                                                                                                    Laboratorium
                                                                                                                                    Paket Laboratorium
                                                                                                                                    Obat
                                                                                                                                    Konfigurasi Dashboard Antrian
                                                                                                                                    Konfigurasi Form
                                                                                                                                    Konfigurasi Laboratorium
                                                                                                                                    SMS Konfigurasi
                                                                                                                                    Konfigurasi Pembulatan
                                                                                                                                    Asupan Makanan
                                                                                                                                    Industri
                                                                                                                                    PBF
                                                                                                                                                                                Komponen Tarif
                                                                                                                                    Metode Pembayaran
                                                                                                                                                                                Diagnosa
                                                                                                                                    Template Print
                                                                                                                                                                      Data Dasar Puskesmas
                                        
                                                                                    BPJS
                                                                                        Pengaturan BPJS
                                                                                                                                    RS Provider BPJS
                                                                                                                                    Poli FKTL BPJS
                                                                                                                                    Dokter BPJS
                                                                                                                                    Tindakan BPJS
                                                                                                                                    Laboratorium BPJS
                                                                                                                                    Obat BPJS
                                                                                                                                    Ruangan BPJS
                                                                                                                                    Pengaturan Bridging BPJS
                                                                                                                                    Sinkron BPJS
                                                                                                                                    Sinkron BPJS Jejaring
                                                                                                                                
                                        

                                        
                                                                                    SISRUTE
                                                                                        Konfigurasi
                                                                                                                                    Tujuan Rujukan
                                                                                                                                    Kriteria Khusus
                                                                                       
                                                                                        SDM
                                                                                                                                    Ruang Perawatan
                                                                                                                                    Pelayanan
                                                                                                                                    Sarana
                                                                                                                                    Kelas Perawatan
                                                                                                                                    Alkes
                                                                                    
                                                                                    Master Wilayah
                                                                                        Wilayah Kerja
                                                                                                                                    Wilayah Kerja Posyandu
                                                                                    
                                                                                    SIPTM
                                                                                        Puskesmas
                                                                                                                                    Diagnosis
                                                                                                                                    Sinkron SIPTM
                                                                                    
                                        
                                                                                    Scan
                                                                                        Scan QRCode
                                                                                                                                                                        Antrian
                                                                                        Antrian
                                                                                                                                                                        Speaker
                                                                                        Speaker
                                                                                                                                                                
                                
                            
                                                            
                                    GIS 
                                    
                                                                                Pasien
                                                                                                                        Penyakit
                                                                            
                                
                                                                                        
                                    Laporan 
                                    
                                                                                    Grafik
                                            Grafik
                                                                                                                            Dashboard SIP
                                        
                                                                                    Monitoring
                                                                                                                                    Penggunaan Fitur
                                                                                    
                                                                                    Laporan Harian
                                                                                                                                    Pendapatan Tindakan
                                                                                                                                    Kunjungan Pasien
                                                                                                                                    Kunjungan Pasien BPJS
                                                                                                                                    Pelayanan Pasien
                                                                                                                                    Pemeriksaan Medis
                                                                                                                                    Pelayanan Resep
                                                                                                                                    Pengeluaran Obat Pasien
                                                                                                                                    Pengeluaran Alat Kesehatan
                                                                                                                                    Pelayanan Laboratorium
                                                                                                                                    Rujukan Internal
                                                                                                                                    Rujukan External
                                                                                                                                    
                                                                                                    Tindakan Dokter/Perawat
                                                                                            
                                                                                                                                    Pemeriksaan Laboratorium
                                                                                                                                    Kinerja Puskesmas
                                                                                                                                    Pelayanan Ruangan
                                                                                                                                    Penyakit
                                                                                                                                    Asuhan Keperawatan
                                                                                                                                    Kunjungan PTM
                                                                                                                                    PKPR
                                                                                                                                    Alat Kesehatan Terbanyak
                                                                                                                                    Tindakan Terbanyak
                                                                                                                                    Obat Kadaluarsa
                                                                                                                                    Laporan Pendapatan
                                                                                                                                    Laporan Pendapatan Pendaftaran 
                                             
                                                                                                                                                                                        Laporan Pendapatan Kasir
                                                                                                                                                                                    Laporan Transaksi Kasir
                                                                                                                                                                                Laporan Skrining COVID-19
                                                                                                                                    Laporan Pengeluaran Obat
                                                                                                                                    Laporan Stok Opname
                                                                                                                                        Laporan Jasa Tenaga Medis
                                                                                                                                        Laporan Hepatitis C
                                                                                                                                        Laporan Diare
                                                                                                                                        Laporan Hepatitis B
                                                                                                                                        Pendapatan Laboratorium
                                                                                    
                                                                                    Laporan Mingguan
                                                                                        Laporan PWS Penyakit
                                                                                    
                                                                                    Laporan Bulanan
                                                                                        Laporan
                                                                                                                                    Data Dasar
                                                                                                                                    Kirim Report
                                                                                                                                    Monitoring Laporan
                                                                                                                                    SP3 LB1
                                                                                                                                    SP3 LB2 (LPLPO)
                                                                                                                                    SP3 LB3
                                                                                                                                    SP3 LB4
                                                                                                                                    UKP-1. Pelayanan Puskesmas
                                                                                                                                    UKP-2. Kesakitan Umum
                                                                                                                                    UKP-3. Kesakitan Gigi dan Mulut
                                                                                                                                    UKP-4. Data Kesakitan Terbanyak
                                                                                                                                    UKP-5. Data Kematian di Puskesmas
                                                                                                                                    UKP-6. LPLPO
                                                                                                                                    UKME-1. Promosi Kesehatan
                                            UKME-2. Kesehatan Lingkungan
                                            UKME-3. Gizi, Kesehatan Ibu dan Kesehatan Anak
                                            UKME-4. Imunisasi
                                            UKME-5. Pengendalian Penyakit Menular
                                            UKME-6. Pengendalian Penyakit Tidak Menular
                                            UKME-7. Perawatan Kesehatan Masyarakat
                                            Penyakit Menular Potensi KLB
                                            Penyakit Potensi KLB Menurut Desa/Kel
                                            Promosi Kesehatan Data Kemitraan
                                            UKMP. Kesehatan Kerja
                                            Program
                                                                                                                                    Pemeriksaan IMS
                                                                                                                                    Laporan Kunjungan Ibu Hamil
                                                                                                                                    Kohort KIA
                                                                                                                                    PKPR
                                                                                                                                    Konseling HIV
                                                                                                                                    Rekapitulasi MTBS MTBM
                                                                                                                                    PTM
                                                                                                                                    Kirim Laporan
                                                                                        Laporan Tahunan
                                            Laporan Eliminasi 3E
                                                                                        Laporan Tahunan
                                            Laporan Tahunan LSD 1
                                                                                                                                    Laporan Tahunan LSD 2
                                                                                                                                    Laporan Tahunan LSD 3
                                                                                                                                                                                
                                                                            
                                
                            
                                                    
                                                               
                        
                            
	.broadcastBox::-webkit-scrollbar-track
    {
        -webkit-box-shadow: inset 0 0 6px rgba(0,0,0,0.3);
        background-color: #F5F5F5;
      border-radius: 5px
    }

    .broadcastBox::-webkit-scrollbar
    {
        width: 10px;
        background-color: #F5F5F5;
      border-radius: 5px
    }

    .broadcastBox::-webkit-scrollbar-thumb
    {
        background-color: black;
        border: 2px solid black;
      border-radius: 5px
    }

    .fasBroadcast{
      font-size: 25pt;
      padding-bottom: 10px;
      color: black;
      margin-right: 40px;
      margin-left: 40px;
    }

    .broadcastBox{
      width: 400px;
      height: 0px;
      border-radius: 10px;
      transition: 0.5s;
      position: absolute;
      overflow-y: scroll;
      padding: 0px;
      left: -360px;
      margin-top: 5px;
      background-color: #F4F4F4;
      -webkit-box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.2);
      -moz-box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.1);
      box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.1);
    }

    .fasBroadcast:hover {
      color: #d63031;
    }


    .displayBroadcast{
      position: relative;
    }

    .contBroadcast{
      position: absolute;
      top: 0;
      width: 100%;
      height: 100%;
      background-color: #F4F4F4;
    }

    .contBroadcast:empty{
      display: none;
    }

    .stickBroadcast{
      text-align: center;  
      display: block;
      font-size: 50pt;
      padding-top: 70px;
      padding-left: 80px
    }

    .stickBroadcast:hover{
      color: black;
    }

    .centBroadcast{
      text-align: center;
      display: block;
    }

    .secBroadcast{
      padding: 25px 10px;
      background-color: #F4F4F4;
      transition: 0.5s;
    }

    .profContBroadcast{
      padding-left: 15px;
    }

    .profileBroadcast{
      -webkit-clip-path: circle(50% at 50% 50%);
      clip-path: circle(50% at 50% 50%);
      width: 75px;
      float: left;
    }

    .txtBroadcast{
      vertical-align: top;
      font-size: 1.25rem;
      padding: 5px 10px 0px 115px;
    }

    .subBroadcast{
      font-size: 1rem;
      color: grey;
    }

    .newBroadcast{
      border-style: none none solid none;
      border-color: red;
    }

    .secBroadcast:hover{
      background-color: #BFBFBF;
    }

    .headerBroadcast{
      padding: 5px;
    }

    .isiContentBroadcast{
      padding: 10px;
    }

    .judulBroadcast{
      padding-bottom: 10px;
    }

    .dotBroadcast {
      height: 9px;
      width: 9px;
      background-color: #9D2D15;
      border-radius: 50%;
      display: inline-block;
    }

    .containerNotifBroadcast img {
      transition: transform 0.25s ease;
      cursor: zoom-in;
    }

    .zoomCheckNotif:checked ~ label > img {
      transform: scale(2);
      cursor: zoom-out;
    }


  



    var openBroadcast = &quot;true&quot;
    let numClicks = 0;
    function showHideBroadcast() {
        if (openBroadcast == &quot;false&quot;) {
          $(&quot; , &quot;'&quot; , &quot;.broadcastBox&quot; , &quot;'&quot; , &quot;).removeAttr(&quot;style&quot;);
          openBroadcast = &quot;true&quot;;
          return false
        } 

        numClicks++;
        if (numClicks === 1) {
          singleClickTimer = setTimeout(() => {
            numClicks = 0;
          }, 400);
        } else if (numClicks === 2) {
          clearTimeout(singleClickTimer);
          numClicks = 0;
          return false
        }

        $.ajax({
            url: &quot;https://demo.epuskesmas.id/notifikasi/getform&quot;,
            method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
            dataType: &quot;json&quot;,
            data: {
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(data) {
                $(&quot;#isiNotifBroadcast&quot;).html(data.data)
                setTimeout(function (){
                      if(openBroadcast == &quot;true&quot;) {
                          $(&quot; , &quot;'&quot; , &quot;.broadcastBox&quot; , &quot;'&quot; , &quot;).css({
                              height: &quot;60vh&quot;
                          });
                          openBroadcast = &quot;false&quot;;
                      }
                }, 100);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                console.log(xhr.status);
                console.log(thrownError);
            }
        });

        
    }


    function setPilihanBroadcast() {
        var broadcastPilih = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;broadcastPilihan&quot;]&quot; , &quot;'&quot; , &quot;).val()

        $.ajax({
            url: &quot;https://demo.epuskesmas.id/notifikasi/getform&quot;,
            method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
            dataType: &quot;json&quot;,
            data: {
                pilihanBroadcast: broadcastPilih,
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(data) {
                $(&quot; , &quot;'&quot; , &quot;#contBroadcast&quot; , &quot;'&quot; , &quot;).html(&quot;&quot;)
                $(&quot; , &quot;'&quot; , &quot;#contBroadcast&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&lt;br/>&lt;br/>&lt;span class=&quot;loading&quot; style=&quot;background-color:#F4F4F4!important;padding-top: 30px;padding-left: 90%;&quot;>&amp;nbsp;&lt;/span>&quot; , &quot;'&quot; , &quot;)
                setTimeout(function (){
                  $(&quot; , &quot;'&quot; , &quot;#contBroadcast&quot; , &quot;'&quot; , &quot;).html(data.data)
                }, 300);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                console.log(xhr.status);
                console.log(thrownError);
            }
        }); 
    }

    function bacaSelengkapnyaBroadcast(ids) {
      $(&quot; , &quot;'&quot; , &quot;.broadcastBox&quot; , &quot;'&quot; , &quot;).removeAttr(&quot;style&quot;);
      openBroadcast = &quot;true&quot;;

      $.blockUI({
        css: {
            border: &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;,
            padding: &quot; , &quot;'&quot; , &quot;15px&quot; , &quot;'&quot; , &quot;,
            backgroundColor: &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;,
            &quot; , &quot;'&quot; , &quot;-webkit-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
            &quot; , &quot;'&quot; , &quot;-moz-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
            opacity: .5,
            fontSize: &quot; , &quot;'&quot; , &quot;8px&quot; , &quot;'&quot; , &quot;,
            color: &quot; , &quot;'&quot; , &quot;#fff&quot; , &quot;'&quot; , &quot;
            },
            message: &quot; , &quot;'&quot; , &quot;&lt;h3>Memuat....&lt;/h3>&quot; , &quot;'&quot; , &quot;
        });

      $.unblockUI()

      $.ajax({
          url: &quot;https://demo.epuskesmas.id/notifikasi/show&quot;,
          method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
          dataType: &quot;json&quot;,
          data: {
              ids: ids,
              _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
          },
          success: function(data) {
              $(&quot;#modal_notif_broadcast_notifikasi&quot;).modal({ backdrop: &quot; , &quot;'&quot; , &quot;static&quot; , &quot;'&quot; , &quot;});
              $(&quot;#modal_notif_broadcast_notifikasi .modal-dialog&quot;).addClass(&quot;modal-lg&quot;);
              $(&quot; , &quot;'&quot; , &quot;#modal_notif_broadcast_notifikasi .modal-title&quot; , &quot;'&quot; , &quot;).html(data.tipe);
              $(&quot; , &quot;'&quot; , &quot;#modal_notif_broadcast_notifikasi .modal-form&quot; , &quot;'&quot; , &quot;).html(data.data);
              console.log(data.ressRead)
              if (data.ressRead == false) {
                $(&quot;#dotBroadcast&quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;)
              }
          },
          error: function (xhr, ajaxOptions, thrownError) {
              alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
              console.log(xhr.status);
              console.log(thrownError);
          }
      });   
    }

    function skemaAction() {
      let skema = $(&quot;#skema&quot;).val()
      $(&quot;#skemaAction&quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;)
      var broadcastPilih = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;broadcastPilihan&quot;]&quot; , &quot;'&quot; , &quot;).val()
      $.ajax({
          url: &quot;https://demo.epuskesmas.id/notifikasi/getformother&quot;,
          method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
          dataType: &quot;json&quot;,
          data: {
              skema: skema,
              pilihanBroadcast: broadcastPilih,
              _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
          },
          success: function(data) {
            setTimeout(function (){
            $(&quot;#skemaAction&quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;)
            let totalSkema = Number(skema) + 1
              $(&quot;#skema&quot;).val(totalSkema)
              $(&quot;#isiDetailNotif&quot;).append(data.data)

              if(data.nextAction != &quot;true&quot;) {
                $(&quot;#skemaAction&quot;).addClass(&quot;hidden&quot;)
              }
            }, 300);
          },
          error: function (xhr, ajaxOptions, thrownError) {
              $(&quot;#skemaAction&quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;)
              alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
              console.log(xhr.status);
              console.log(thrownError);
          }
      }); 
    }
                        
                        
                                                             
                                                                        
                                         Belum Bridging
                                        8      
                                    
                                                                                                                        
                                                    
                                
                                    Darel PUSKESMAS PUSKESMAS1 
                                
                                
                                    Profil
                                                                            Update
                                                                        Umpan Balik
                                                                            Bantuan
                                                                        
                                    
                                        
                                            Keluar
                                        

                                        
                                            
                                        
                                    
                                
                            
                                            
                
            
        

        
        
        
        

        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        

        
                    backgroundChart = &quot;images/gis/bg-grafik.png&quot;;
                
        
        

        
        
            


    Welcome
                    Dashboard Utama
                        Dashboard Antrian
                                        Dashboard Rawat Inap
                                    Dashboard SIP
                                    Dashboard KIA
            
        Tampilkan Data
            Tampilkan Data
                        Tampilkan Data
                
        
            
                ×
                Pilih Tanggal
                
                
                    
                    
                        
                            
                            
                        
                    
                    
                        Tampilkan
                    
                
                
                    
                
            

        
    
    


    
        
            
                
                
                                                                        PUSKESMAS PUSKESMAS1
                                                                PT. Infokes Indonesia Infokes e-Health Solutions
                
            
            
            
            
            
            
                                                                                        Selamat Datang di ePuskesmas NG!
                                                                
                    
                        Sistem Informasi Manajemen Pelayanan kesehatan
                        Berbasis Teknologi Informasi untuk Mewujudkan Pelayanan Prima 
                        disetiap unit kesehatan Indonesia (Puskesmas, Klinik, Praktek Dokter, Apotek, Posyandu).
                    
                
                                
            
        
        
            
                                                            
                       
                
                            
        
    


    
        
            
                
                     Pasien Data Lengkap -
                    
                        
                    
                    Data pasien terverifikasi lengkap
                
            
            
                
                     Pasien Sudah Dilayani -
                    
                        
                    
                    Pasien yang sudah dilayani di semua pelayanan
                
            
                        
                
                     Resep Obat Diberikan -
                    
                        
                    
                    Resep obat yang sudah diproses di apotek
                
            
                        
                
                     Pasien Sudah Membayar -
                    
                        
                    
                    Pasien Umum yang membayar di Kasir
                
            
            
                
                     Pasien BPJS Sudah Dilayani -
                    
                        
                    
                    Jumlah Pasien BPJS yang sudah dilayani
                
            
            
                
                     Pasien BPJS Sudah Bridging -
                    
                        
                    
                    Pasien BPJS yang sudah Bridging ke PCare
                
            
        
    
    
        
            
                
            
            
                
                

            
        
    


    
        
        
        Fullscreen
    
    
        
            Dashboard antrian realtime belum di setting
                                                                                                      
                            
                                KB
                                
                                    
                                        
                                            
                                                                                                                                                                                                 
                                                 
                                                 
                                                 
                                                 
                                                                                            
                                            
                                                
                                            
                                        
                                    
                                
                                  Total Antrian : 0 Pasien
                            
                        
                                                                                
    


    
        
        
        Fullscreen
    
    
        
            Dashboard rawat inap realtime belum di setting
                            
                    
                        
                            
                                Inap Umum
                                Mawar Hitam
                            
                        
                        
                        
                            
                                
                                    
                                        Tersedia : 
                                    
                                
                            
                            
                                
                                    
                                        Terisi : 
                                    
                                
                            
                        
                    
                
                            
                    
                        
                            
                                Inap Umum
                                bunga
                            
                        
                        
                        
                            
                                
                                    
                                        Tersedia : 
                                    
                                
                            
                            
                                
                                    
                                        Terisi : 
                                    
                                
                            
                        
                    
                
                            
                    
                        
                            
                                Inap Umum
                                Melati
                            
                        
                        
                        
                            
                                
                                    
                                        Tersedia : 
                                    
                                
                            
                            
                                
                                    
                                        Terisi : 
                                    
                                
                            
                        
                    
                
                            
                    
                        
                            
                                Inap Umum
                                Melati
                            
                        
                        
                        
                            
                                
                                    
                                        Tersedia : 
                                    
                                
                            
                            
                                
                                    
                                        Terisi : 
                                    
                                
                            
                        
                    
                
                    
    


            function openTab(evt, Name) {
            var i, tabcontent, tablinks;
            tabcontent = $(&quot;.tabcontent&quot;);
            for (i = 0; i &lt; tabcontent.length; i++) {
                tabcontent[i].style.display = &quot;none&quot;;
            }
            tablinks = $(&quot;.tablinks&quot;);
            for (i = 0; i &lt; tablinks.length; i++) {
                tablinks[i].className = tablinks[i].className.replace(&quot; active&quot;, &quot;&quot;);
            }
            document.getElementById(Name).style.display = &quot;block&quot;;

            $(&quot;button[data-id=tampilkan_data_utama]&quot;).hide();
            $(&quot;button[data-id=tampilkan_data_antrian]&quot;).hide();
            $(&quot;button[data-id=tampilkan_data_ranap]&quot;).hide();
            $(&quot;h4[id=statistik]&quot;).hide();
            if(Name == &quot; , &quot;'&quot; , &quot;Dashboard&quot; , &quot;'&quot; , &quot;){
                $(&quot;button[data-id=tampilkan_data_utama]&quot;).show();
                $(&quot;h4[id=statistik]&quot;).show();
            }else if(Name == &quot; , &quot;'&quot; , &quot;Antrian&quot; , &quot;'&quot; , &quot;){
                $(&quot;button[data-id=tampilkan_data_antrian]&quot;).show();
            }else if(Name == &quot; , &quot;'&quot; , &quot;Ranap&quot; , &quot;'&quot; , &quot;){
                $(&quot;button[data-id=tampilkan_data_ranap]&quot;).show();
            }
        }
    
    function openUrl(url){
        window.open(url, &quot; , &quot;'&quot; , &quot;_self&quot; , &quot;'&quot; , &quot;);
    }

    $(() => {
                    switch(window.location.hash){
                                case &quot; , &quot;'&quot; , &quot;#Antrian&quot; , &quot;'&quot; , &quot;:
                    $(&quot; , &quot;'&quot; , &quot;div[class=tab]&quot; , &quot;'&quot; , &quot;).find(&quot;#btn_dashboard_antrian&quot;).click();
                break;
                                                case &quot; , &quot;'&quot; , &quot;#Ranap&quot; , &quot;'&quot; , &quot;:
                    $(&quot; , &quot;'&quot; , &quot;div[class=tab]&quot; , &quot;'&quot; , &quot;).find(&quot;#btn_dashboard_ranap&quot;).click();
                break;
                                                case &quot; , &quot;'&quot; , &quot;#Dashboard&quot; , &quot;'&quot; , &quot;:
                    $(&quot; , &quot;'&quot; , &quot;div[class=tab]&quot; , &quot;'&quot; , &quot;).find(&quot;#btn_dashboard_utama&quot;).click();
                break;
                                default:
                    $(&quot; , &quot;'&quot; , &quot;div[class=tab]&quot; , &quot;'&quot; , &quot;).find(&quot;#btn_dashboard_welcome&quot;).click();
            }
                $(&quot;div[id=&quot; , &quot;'&quot; , &quot;Dashboard&quot; , &quot;'&quot; , &quot;] div[id*=box_] div&quot;).mouseover(function() {
            $(this).css(&quot; , &quot;'&quot; , &quot;cursor&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;pointer&quot; , &quot;'&quot; , &quot;);
        });
                                    showUpdateLog();
                                        setTimeout(function() {
                    showNotif();
                }, 2000);
                    
                                                           gagalBridgingCount();
                                            var cek = &quot;&quot;;
        if (cek) {
            var logo_ponpes = &quot; , &quot;'&quot; , &quot;/images/portal/logo/&quot; , &quot;'&quot; , &quot;+localStorage.getItem(&quot;img_logo_ponpes&quot;);
            $(&quot; , &quot;'&quot; , &quot;img#logo-portal&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;src&quot; , &quot;'&quot; , &quot;, logo_ponpes);
        }
    });
    


    var date = new Date();
    var tanggal_now = new Date(date.getFullYear(), date.getMonth(), date.getDate()-1, 23, 59, 59, 59);
    $(&quot; , &quot;'&quot; , &quot;.date-custom&quot; , &quot;'&quot; , &quot;).datetimepicker({
        locale:&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;,
        format:&quot; , &quot;'&quot; , &quot;DD-MM-YYYY&quot; , &quot;'&quot; , &quot;,
        maxDate:tanggal_now,
        defaultDate: null,
        useCurrent: false,
        ignoreReadonly: true
    });

    $(`#showData`).bind(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function() {
        $(`#jml_daftar, #jml_dilayani, #jml_resep, #jml_bayar, #jml_bpjs, #jml_bridging, #myChart, #myChart2, #myChart3`).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;#box_daftar, #box_dilayani, #box_resep, #box_bayar, #box_bpjs, #box_bridging&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;);
        loadPendaftaran();
        $(`#closeMyModal`).click();
    });

        if (&quot; , &quot;'&quot; , &quot;e-Puskesmas&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;e-Bidan Personal&quot; , &quot;'&quot; , &quot;) {
        var ctx = document.getElementById(&quot; , &quot;'&quot; , &quot;myChart&quot; , &quot;'&quot; , &quot;);
        var myChart = new Chart(ctx, {
            type: &quot; , &quot;'&quot; , &quot;horizontalBar&quot; , &quot;'&quot; , &quot;,
            data: {
                labels: [],
                datasets: []
            },
            options: {
                scales: {
                    yAxes: [{
                        ticks: {
                            beginAtZero: true
                        }
                    }]
                },
                title: {
                    display: true,
                    text: &quot;10 Penyakit Terbanyak Hari Ini&quot;,
                }
            }
        });
    }else{
        var ctx = document.getElementById(&quot; , &quot;'&quot; , &quot;myChart&quot; , &quot;'&quot; , &quot;);
        var myChart = new Chart(ctx, {
            type: &quot; , &quot;'&quot; , &quot;bar&quot; , &quot;'&quot; , &quot;,
            data: {
                labels: [],
                datasets: []
            },
            options: {
                scales: {
                    yAxes: [{
                        ticks: {
                            beginAtZero: true
                        }
                    }]
                },
                title: {
                    display: true,
                    text: &quot;Pasien Poli / Ruangan Hari Ini&quot;,
                }
            }
        });
    }

    var ctx2 = document.getElementById(&quot; , &quot;'&quot; , &quot;myChart2&quot; , &quot;'&quot; , &quot;);
    var myChart2 = new Chart(ctx2, {
        type: &quot; , &quot;'&quot; , &quot;pie&quot; , &quot;'&quot; , &quot;,
        data: {
            labels: [&quot; , &quot;'&quot; , &quot;Lama&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Baru&quot; , &quot;'&quot; , &quot;],
            datasets: [{
                data: [],
                backgroundColor: [&quot; , &quot;'&quot; , &quot;#5555FF&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#55FF55&quot; , &quot;'&quot; , &quot;],
            }]
        },
        options: {
            legend: {
                display: true,
            },
            title: {
                display: true,
                text: &quot;Kunjungan Pasien Hari Ini&quot;,
            }
        }
    });
    var ctx3 = document.getElementById(&quot; , &quot;'&quot; , &quot;myChart3&quot; , &quot;'&quot; , &quot;);
    var myChart3 = new Chart(ctx3, {
        type: &quot; , &quot;'&quot; , &quot;line&quot; , &quot;'&quot; , &quot;,
        data: {
            labels: [],
            datasets: [{
                label: &quot;Jumlah Kunjungan&quot;,
                backgroundColor: &quot;rgba(38, 185, 154, 0.31)&quot;,
                borderColor: &quot;rgba(38, 185, 154, 0.7)&quot;,
                pointBorderColor: &quot;rgba(38, 185, 154, 0.7)&quot;,
                pointBackgroundColor: &quot;rgba(38, 185, 154, 0.7)&quot;,
                pointHoverBackgroundColor: &quot;#fff&quot;,
                pointHoverBorderColor: &quot;rgba(220,220,220,1)&quot;,
                data: [],
            }]
        },
        options: {
            legend: {
                display: false,
            },
            title: {
                display: true,
                text: &quot;Kunjungan 7 Hari Kebelakang&quot;,
            }
        }
    });
    
    function loadPendaftaran(callback) {
        $(&quot; , &quot;'&quot; , &quot;#jml_daftar&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;pendaftaran&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                $(&quot; , &quot;'&quot; , &quot;#statistik&quot; , &quot;'&quot; , &quot;).html(&quot;Statistik : &quot; + $(&quot;#tanggal_custom&quot;).val());
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_daftar&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/pasien&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_daftar&quot; , &quot;'&quot; , &quot;).html(result.pendaftaran.current+&quot;/&quot;+result.pendaftaran.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_daftar&quot;).html(result.pendaftaran.percent+&quot;%&quot;).css(&quot;width&quot;, result.pendaftaran.percent+&quot;%&quot;);
                loadPelayanan();
            }
        });
    }

    function loadPelayanan() {
        $(&quot; , &quot;'&quot; , &quot;#jml_dilayani&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;pelayanan&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_dilayani&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/pendaftaran?tanggal=&quot; , &quot;'&quot; , &quot;+$(&quot;#tanggal_custom&quot;).val()+&quot; , &quot;'&quot; , &quot;&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_dilayani&quot; , &quot;'&quot; , &quot;).html(result.pelayanan.current+&quot;/&quot;+result.pelayanan.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_dilayani&quot;).html(result.pelayanan.percent+&quot;%&quot;).css(&quot;width&quot;, result.pelayanan.percent+&quot;%&quot;);
                if (&quot; , &quot;'&quot; , &quot;e-Puskesmas&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;e-Bidan Personal&quot; , &quot;'&quot; , &quot;) {
                    loadObatPasien();
                } else {
                    loadResep();
                }
            }
        });
    }

    function loadResep() {
        $(&quot; , &quot;'&quot; , &quot;#jml_resep&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;resep&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_resep&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/resep?tanggal=&quot; , &quot;'&quot; , &quot;+$(&quot;#tanggal_custom&quot;).val()+&quot; , &quot;'&quot; , &quot;&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_resep&quot; , &quot;'&quot; , &quot;).html(result.resep.current+&quot;/&quot;+result.resep.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_resep&quot;).html(result.resep.percent+&quot;%&quot;).css(&quot;width&quot;, result.resep.percent+&quot;%&quot;);
                loadBayar();
            }
        });
    }

    function loadObatPasien() {
        $(&quot; , &quot;'&quot; , &quot;#jml_obatpasien&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;obatpasien&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_obatpasien&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/obatpasien?tanggal=&quot; , &quot;'&quot; , &quot;+$(&quot;#tanggal_custom&quot;).val()+&quot; , &quot;'&quot; , &quot;&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_obatpasien&quot; , &quot;'&quot; , &quot;).html(result.obatpasien.current+&quot;/&quot;+result.obatpasien.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_obatpasien&quot;).html(result.obatpasien.percent+&quot;%&quot;).css(&quot;width&quot;, result.obatpasien.percent+&quot;%&quot;);
                loadBayar();
            }
        });
    }

    function loadBayar() {
        $(&quot; , &quot;'&quot; , &quot;#jml_bayar&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;bayar&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_bayar&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/pembayaran?tanggal=&quot; , &quot;'&quot; , &quot;+$(&quot;#tanggal_custom&quot;).val()+&quot; , &quot;'&quot; , &quot;&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_bayar&quot; , &quot;'&quot; , &quot;).html(result.pembayaran.current+&quot;/&quot;+result.pembayaran.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_bayar&quot;).html(result.pembayaran.percent+&quot;%&quot;).css(&quot;width&quot;, result.pembayaran.percent+&quot;%&quot;);
                loadBpjs();
            }
        });
    }

    function loadBpjs() {
        $(&quot; , &quot;'&quot; , &quot;#jml_bpjs&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;bpjs&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_bpjs&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/pendaftaran?asuransi_id=0001&amp;tanggal=&quot; , &quot;'&quot; , &quot;+$(&quot;#tanggal_custom&quot;).val()+&quot; , &quot;'&quot; , &quot;&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_bpjs&quot; , &quot;'&quot; , &quot;).html(result.bpjs.current+&quot;/&quot;+result.bpjs.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_bpjs&quot;).html(result.bpjs.percent+&quot;%&quot;).css(&quot;width&quot;, result.bpjs.percent+&quot;%&quot;);
                loadBridging(result.bpjs.count);
            }
        });
    }

    function loadBridging(total) {
        $(&quot; , &quot;'&quot; , &quot;#jml_bridging&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;bridging&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                total: total,
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                if(window.location.pathname == &quot; , &quot;'&quot; , &quot;/home&quot; , &quot;'&quot; , &quot;){
                    $(&quot; , &quot;'&quot; , &quot;#box_bridging&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;ondblclick&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;openUrl(&quot;https://demo.epuskesmas.id/sinkronbpjs?search[tanggal]=&quot; , &quot;'&quot; , &quot;+$(&quot;#tanggal_custom&quot;).val()+&quot; , &quot;'&quot; , &quot;&quot;)&quot; , &quot;'&quot; , &quot;);
                }
                $(&quot; , &quot;'&quot; , &quot;#jml_bridging&quot; , &quot;'&quot; , &quot;).html(result.bridgingbpjs.current+&quot;/&quot;+result.bridgingbpjs.count).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot;#bar_bridging&quot;).html(result.bridgingbpjs.percent+&quot;%&quot;).css(&quot;width&quot;, result.bridgingbpjs.percent+&quot;%&quot;);
                loadDataBar();
            }
        });
    }

    function loadDataBar() {
        $(`#myChart`).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;bar&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                $(`#myChart`).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                myChart.data.datasets = result;
                if (&quot; , &quot;'&quot; , &quot;e-Puskesmas&quot; , &quot;'&quot; , &quot; == &quot; , &quot;'&quot; , &quot;e-Bidan Personal&quot; , &quot;'&quot; , &quot;) {
                    myChart.options.title.text = &quot;10 Penyakit Terbanyak &quot; + $(&quot;#tanggal_custom&quot;).val();
                } else {
                    myChart.options.title.text = &quot;Pasien Poli / Ruangan &quot; + $(&quot;#tanggal_custom&quot;).val();
                }
                myChart.update();
                loadDataPie();
            }
        });
    }

    function loadDataPie() {
        $(`#myChart2`).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;pie&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                $(`#myChart2`).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                myChart2.data.datasets[0].data = [result.lama, result.baru];
                myChart2.options.title.text = &quot;Kunjungan Pasien &quot; + $(&quot;#tanggal_custom&quot;).val();
                myChart2.update();
                loadDataLine();
            }
        });
    }

    function loadDataLine() {
        $(`#myChart3`).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                load: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                id: $(&quot;#puskesmas_id&quot;).val(),
                type: &quot; , &quot;'&quot; , &quot;line&quot; , &quot;'&quot; , &quot;,
                tanggal: $(&quot;#tanggal_custom&quot;).val(),
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(result) {
                $(`#myChart3`).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                myChart3.data.labels = result.days;
                myChart3.data.datasets[0].data = result.data;
                myChart3.options.title.text = &quot;Kunjungan 7 Hari Kebelakang dari &quot; + $(&quot;#tanggal_custom&quot;).val();
                myChart3.update();
            }
        });
    }


        function loadDataAntrian() {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                antrian: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(response) {
                if(response.ruangan.length){
                    response.ruangan.forEach(function(result, key) {
                        $(&quot;#P0000000001&quot;+result.id+&quot;1&quot;).html(&quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot;);
                        $(&quot;#P0000000001&quot;+result.id+&quot;2&quot;).html(&quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot;);
                        $(&quot;#P0000000001&quot;+result.id+&quot;3&quot;).html(&quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot;);
                        $(&quot;#P0000000001&quot;+result.id+&quot;4&quot;).html(&quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot;);
                        $(&quot;#P0000000001&quot;+result.id+&quot;5&quot;).html(&quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot;);
                        if(response.data.length){
                            if(response.data[key].length){
                                response.data[key].forEach(function(result2, key2) {
                                    let icon = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                                    switch (result2.pendaftaran.reg_type) {
                                        case &quot; , &quot;'&quot; , &quot;mobile&quot; , &quot;'&quot; , &quot;:
                                            icon = &quot; , &quot;'&quot; , &quot;icon-mobile&quot; , &quot;'&quot; , &quot;
                                            break;
                                        case &quot; , &quot;'&quot; , &quot;kiosk&quot; , &quot;'&quot; , &quot;:
                                            icon = &quot; , &quot;'&quot; , &quot;icon-kiosk&quot; , &quot;'&quot; , &quot;
                                            break;
                                        case &quot; , &quot;'&quot; , &quot;sms&quot; , &quot;'&quot; , &quot;:
                                            icon = &quot; , &quot;'&quot; , &quot;icon-sms&quot; , &quot;'&quot; , &quot;
                                            break;
                                        default:
                                            icon = &quot; , &quot;'&quot; , &quot;icon-puskesmas&quot; , &quot;'&quot; , &quot;
                                    }
                                    $(&quot;#P0000000001&quot;+result2.ruangan_id+(key2+1)).html(&quot; , &quot;'&quot; , &quot;&lt;i class=&quot;epus-font &quot; , &quot;'&quot; , &quot;+icon+&quot; , &quot;'&quot; , &quot;&quot;>&lt;/i> &lt;strong>&quot; , &quot;'&quot; , &quot;+ result2.antrean +&quot; , &quot;'&quot; , &quot; - &quot; , &quot;'&quot; , &quot;+ result2.pendaftaran.pasien.nama.substring(0, 15) +&quot; , &quot;'&quot; , &quot;&lt;/strong>&quot; , &quot;'&quot; , &quot;);
                                });
                            }
                            $(&quot;#P0000000001&quot;+result.id+&quot;TOTAL&quot;).html(response.count[key]);
                        }
                    });
                }
            }
        });
    }
            function loadDataRanap() {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/home&quot;,
            method: &quot;GET&quot;,
            data: {
                ranap: &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;,
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
            },
            success: function(response) {
                if(response.length > 0){
                    $.each(response, function (key, val) { 
                        let data = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                        $.each(val.bed_details, function (keyBed, valBed) { 
                            let txtColor = &quot; , &quot;'&quot; , &quot;primary&quot; , &quot;'&quot; , &quot;;
                            if (valBed.hasOwnProperty(&quot; , &quot;'&quot; , &quot;pelayanan&quot; , &quot;'&quot; , &quot;) &amp;&amp; valBed.pelayanan !== null 
                                &amp;&amp; valBed.pelayanan.hasOwnProperty(&quot; , &quot;'&quot; , &quot;pendaftaran&quot; , &quot;'&quot; , &quot;) &amp;&amp; valBed.pelayanan.pendaftaran !== null) {
                                txtColor = &quot; , &quot;'&quot; , &quot;danger&quot; , &quot;'&quot; , &quot;;
                            }
                            data += &quot; , &quot;'&quot; , &quot;&lt;i class=&quot;fa fa-bed text-&quot; , &quot;'&quot; , &quot;+ txtColor +&quot; , &quot;'&quot; , &quot;&quot; style=&quot;margin:3px;&quot; title=&quot;&quot; , &quot;'&quot; , &quot;+ valBed.nama +&quot; , &quot;'&quot; , &quot;&quot;>&lt;/i>&quot; , &quot;'&quot; , &quot;;
                        });
                        $(&quot;#P0000000001&quot; + val.ruangan_id + val.id).html(data);
                        $(&quot;#P0000000001&quot; + val.ruangan_id + val.id + &quot;TERISI&quot;).html(val.terisi);
                        $(&quot;#P0000000001&quot; + val.ruangan_id + val.id + &quot;TERSEDIA&quot;).html(val.tersedia);
                    });
                }
            }
        });
    }
            
        
            
                 
            
        
        
        
        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                    
                
            
        

        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                        
                    
                
            
        

        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                    
                
            
        
        
            
                
                    
                        ×
                          Notifikasi
                    
                    
                        
                        
                    
                
            
        
                                                
	#modalBantuan {
        position: fixed;
        z-index: 99998; 
        height: 52px; 
        width: 52px;
        background-color: #25d366;
        text-align: center;
        bottom: 5%;
        left: 2%;
        border-radius: 50%;
        font-size: 40px;
    }

    #modalBantuan > i {
        color: white;
        padding-top: 5px;
    }






    function openBantuan(){
        Swal.fire({
            title: &quot; , &quot;'&quot; , &quot;Apakah anda membutuhkan bantuan dari Team Customer Support?&quot; , &quot;'&quot; , &quot;,
            type: &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;,
            showCancelButton: true,
            confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
            cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
            confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya&quot; , &quot;'&quot; , &quot;,
            cancelButtonText: &quot; , &quot;'&quot; , &quot;Tidak&quot; , &quot;'&quot; , &quot;
        }).then((result) => {
            if (result.value) {
                let notif_wa = &quot;https://api.whatsapp.com/send/?phone=6281277073300&amp;text=Hi,%20Saya%20pengguna%20ePuskesmas:%20,%20%20(darel)%20-%20P0000000001%20PUSKESMAS1%20-%20KABUPATEN%20BANDUNG%20memerlukan%20bantuan%20anda!&amp;app_absent=0&quot;
                window.open(notif_wa);
            }
        });
    }

    $(&quot;#modalBantuan&quot;).mousedown(function() {
        isDragging = false;
    }).mousemove(function() {
        isDragging = true;
    }).mouseup(function() {
        var wasDragging = isDragging;
        isDragging = false;
        if (!wasDragging) {
            openBantuan();
        }
    });

    dragElement(document.getElementById(&quot;modalBantuan&quot;));

    function dragElement(elmnt) {
      var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
      if (document.getElementById(elmnt.id + &quot;header&quot;)) {
        document.getElementById(elmnt.id + &quot;header&quot;).onmousedown = dragMouseDown;
      } else {
        elmnt.onmousedown = dragMouseDown;
      }

      function dragMouseDown(e) {
        e = e || window.event;
        e.preventDefault();
        pos3 = e.clientX;
        pos4 = e.clientY;
        document.onmouseup = closeDragElement;
        document.onmousemove = elementDrag;
      }

      function elementDrag(e) {
        e = e || window.event;
        e.preventDefault();
        pos1 = pos3 - e.clientX;
        pos2 = pos4 - e.clientY;
        pos3 = e.clientX;
        pos4 = e.clientY;
        elmnt.style.top = (elmnt.offsetTop - pos2) + &quot;px&quot;;
        elmnt.style.left = (elmnt.offsetLeft - pos1) + &quot;px&quot;;
      }

      function closeDragElement() {
        document.onmouseup = null;
        document.onmousemove = null;
      }
    }

    var modalBantuanDrag = document.getElementById(&quot; , &quot;'&quot; , &quot;modalBantuan&quot; , &quot;'&quot; , &quot;);
    modalBantuanDrag.addEventListener(&quot; , &quot;'&quot; , &quot;touchmove&quot; , &quot;'&quot; , &quot;, function(event) {
      event.preventDefault();
      if (event.targetTouches.length == 1) {
        var touch = event.targetTouches[0];
        modalBantuanDrag.style.left = touch.pageX + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;;
        modalBantuanDrag.style.top = touch.pageY + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;;
      }
    }, {passive: true});
                
            $(document).ready(function(){
                $(window).scroll(function () {
                    $(&quot; , &quot;'&quot; , &quot;#icon_back&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;margin-top:&quot; , &quot;'&quot; , &quot;+(parseInt(innerHeight)-150)+&quot; , &quot;'&quot; , &quot;px;margin-left:&quot; , &quot;'&quot; , &quot;+(parseInt(innerWidth)-70)+&quot; , &quot;'&quot; , &quot;px;z-index:1000;&quot; , &quot;'&quot; , &quot;);
                    if ($(this).scrollTop() > 125) {
                        $(&quot; , &quot;'&quot; , &quot;#top-link-block&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
                        $(&quot; , &quot;'&quot; , &quot;#top-link-block&quot; , &quot;'&quot; , &quot;).fadeIn();
                    }else{
                        $(&quot; , &quot;'&quot; , &quot;#top-link-block&quot; , &quot;'&quot; , &quot;).fadeOut();
                    }                    
                });

                let message_skrining= &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;;
                if(message_skrining == &quot;error_message&quot;){

                    let text_message = &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;;
                    let text_status = &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;;
                    if(text_status == &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;){
                        text_status = &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;;
                    }
                    if(text_message != &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;){
                        Swal.fire(
                          text_message,
                          &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
                          text_status
                        )
                    }
                }

                $(&quot; , &quot;'&quot; , &quot;[data-toggle=&quot;tooltip&quot;]&quot; , &quot;'&quot; , &quot;).tooltip();


                                                                                        let stt= localStorage.getItem(&quot;count_notif&quot;);
                            if(stt !== &quot;undefined&quot; &amp;&amp; stt !== &quot;0&quot; &amp;&amp; stt !== &quot;null&quot; ){
                                $(&quot;#kolom_gagal_bridging&quot;).html(stt);   
                            }
                                                                            
            });
            
            function aksipesan(){
                Swal.fire({
                    text: &quot;Apakah Anda ingin tutup pesan Webservice bpjs dan tidak di tampilkan lagi ?&quot;,
                    type: &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;,
                    showCancelButton: true,
                    confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
                    cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
                    confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya&quot; , &quot;'&quot; , &quot;,
                    cancelButtonText: &quot; , &quot;'&quot; , &quot;Tutup Sekarang Saja&quot; , &quot;'&quot; , &quot;
                }).then((result) => {
                    if(result.value){
                        setSessionWebSocket(1, &quot;indikatorbpjs&quot;);
                        $(&quot; , &quot;'&quot; , &quot;#notif-connection-bpjs&quot; , &quot;'&quot; , &quot;).hide();
                    }

                    if (result.dismiss == &quot;cancel&quot;) {
                        $(&quot; , &quot;'&quot; , &quot;#notif-connection-bpjs&quot; , &quot;'&quot; , &quot;).hide();
                    }
                });
            }

                        function callAntreanKiosk(nomorAntrean, ruangan_code){
                                return false;
            }
            
            function deletePasienKiosk(pasien_id){
                                return false;
            }

            function deletePendaftaranKiosk(pendaftaran_id){
                                return false;
            }

            function syncPasienKiosk(){
                                return false;
            }

            function syncPendaftaranKiosk(){
                                return false;
            }
        
        
            var date = new Date();
            var tanggal = new Date(date.getFullYear(), date.getMonth(), date.getDate(), 23, 59, 59, 59);
            var bulan = new Date(date.getFullYear(), date.getMonth(), 1, 0, 0, 0);
            var tahun = new Date(date.getFullYear(), 1, 0, 0, 0);
                            var tanggal_min = new Date(date.getFullYear(), date.getMonth(), date.getDate(), 00, 00, 00, 00);
                        var sistole_diastole_anamnesa = &quot;&quot;;
            var default_sistole = &quot;&quot;;
            var default_diastole = &quot;&quot;;
        
        
                    
            
                
            function showUpdateLog(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/updatelog/showupdatelog&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                    },
                    success: function(data) {
                        if(data.title != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; data.title != undefined){
                            $(&quot; , &quot;'&quot; , &quot;#modal .modal-title&quot; , &quot;'&quot; , &quot;).html(data.title);
                            $(&quot; , &quot;'&quot; , &quot;#modal .modal-form&quot; , &quot;'&quot; , &quot;).html(data.form);
                            openmodal(&quot;modal-lg&quot;);
                        }else{
                            if(from_menu){
                                alert(&quot;Belum ada update terbaru saat ini!&quot;,&quot;info&quot;);
                            }
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function showNotif(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/mpuskesmas/shownotif&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                    },
                    success: function(data) {
                        if(data.title != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; data.title != undefined){
                            $(&quot; , &quot;'&quot; , &quot;#modal_notif .modal-title&quot; , &quot;'&quot; , &quot;).html(data.title);
                            $(&quot; , &quot;'&quot; , &quot;#modal_notif .modal-form&quot; , &quot;'&quot; , &quot;).html(data.form);
                            openmodal_notif(&quot;modal-lg&quot;);

                            if (data.notif==&quot; , &quot;'&quot; , &quot;NOTIF_02&quot; , &quot;'&quot; , &quot;) {
                                $(&quot; , &quot;'&quot; , &quot;#btn_close_suspend&quot; , &quot;'&quot; , &quot;).hide();
                            }

                        }else{
                            if(from_menu){
                                alert(&quot;Belum ada update terbaru saat ini!&quot;,&quot;info&quot;);
                            }
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function showNotifBridging(from_menu = false)
            {
                $.blockUI({
                css: {
                    border: &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;,
                    padding: &quot; , &quot;'&quot; , &quot;15px&quot; , &quot;'&quot; , &quot;,
                    backgroundColor: &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;-webkit-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;-moz-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
                    opacity: .5,
                    fontSize: &quot; , &quot;'&quot; , &quot;8px&quot; , &quot;'&quot; , &quot;,
                    color: &quot; , &quot;'&quot; , &quot;#fff&quot; , &quot;'&quot; , &quot;
                    },
                    message: &quot; , &quot;'&quot; , &quot;&lt;h3>Memuat....&lt;/h3>&quot; , &quot;'&quot; , &quot;
                });

                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/home/shownotifbridging&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                    },
                    success: function(data) {
                        $(&quot; , &quot;'&quot; , &quot;#modal_notif_bridging .modal-title&quot; , &quot;'&quot; , &quot;).html(data.title);
                        $(&quot; , &quot;'&quot; , &quot;#modal_notif_bridging .modal-form&quot; , &quot;'&quot; , &quot;).html(data.form);
                        openmodal_notif_bridging(&quot;modal-lg&quot;);
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function gagalBridgingCount(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/home/gagalbridgingcount&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                    },
                    success: function(data) {
                        var x = document.getElementById(&quot;button_gagal_bridging&quot;);
                        if(data > 0){
                            $(&quot;#kolom_gagal_bridging&quot;).html(data);
                            localStorage.setItem(&quot; , &quot;'&quot; , &quot;count_notif&quot; , &quot;'&quot; , &quot;,data);    
                        }else{
                            localStorage.setItem(&quot; , &quot;'&quot; , &quot;count_notif&quot; , &quot;'&quot; , &quot;,&quot;0&quot;);
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

        function showRiwayatPelayanan(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayatpelayanan&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                id: data.id,
                puskesmas: data.puskesmas,
                _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;,
            },
            success: function(data) {
                $(&quot;#modal .modal-title&quot;).html(data.title);
                $(&quot;#modal .modal-form&quot;).html(data.form);
                openmodal(&quot;modal-lg&quot;);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                    console.log(xhr.status);
                    console.log(thrownError);
                }
            });
        }

        function showRiwayatKunjunganBpjs(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayatkunjunganbpjs&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                    id: data.id,
                    _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;,
                },
                success: function(data) {
                    $(&quot;#modal .modal-title&quot;).html(data.title);
                    $(&quot;#modal .modal-form&quot;).html(data.form);
                    openmodal(&quot;modal-lg&quot;);
                },
                error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                }
            });
        }

        function getRiwayatKunjunganBpjs(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/getriwayatkunjunganbpjs&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                    noasuransi: data.noasuransi,
                    _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;,
                },
                success: function(data) {
                    alert(data.message, data.status);
                    $(obj).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                    if(data.status == &quot;success&quot;){
                        window.location.reload(true);
                    }
                },
                error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                    console.log(xhr.status);
                    console.log(thrownError);
                }
            });
        }

        function setSessionWebSocket(action,to = null)
        {
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/home/setsessionwebsocket&quot;,
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                dataType: &quot;json&quot;,
                data: {
                    action: action,
                    for_customsession:to,
                    _token: &quot;fm3YtpjU9kiJQn7LEyO2dLyHKXdIbmLP3GgJAT9s&quot;
                },
                success: function(data) {
                    console.log(&quot;success&quot;);
                },
                error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                }
            });
        }

        
        

        
                        
                    $(`#checkEnvConfig`).html(&quot; , &quot;'&quot; , &quot;&lt;center>Dashboard antrian realtime belum di setting&lt;/center>&quot; , &quot;'&quot; , &quot;);
                    $(`#checkEnvConfigRanap`).html(&quot; , &quot;'&quot; , &quot;&lt;center>Dashboard rawat inap realtime belum di setting&lt;/center>&quot; , &quot;'&quot; , &quot;);
                
                        

/html[1]/body[1]&quot;))]</value>
      <webElementGuid>0e0af3f9-0912-4c7c-8a94-e30c73809247</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
