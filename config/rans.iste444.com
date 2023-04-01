server {
    listen 80;
    listen [::]:80;
    server_name rans.iste444.com;

    root /var/www/rans/public;
    index index.html;

    location / {
        try_files $uri $uri/ =404;
    }
}