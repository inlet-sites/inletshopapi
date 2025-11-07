pub fn reset_password(name: String, id: String, token: String) -> String {
    format!(
        r#"
<p>Hello {name},</p>

<p>We have received a request to reset your password. To do this, simply use the link below and enter your email address.</p>

<p>If you did not make this request, then you can safely ignore this email.</p>

<a href="https://vendor.inlet.shop/password/{id}/{token}">
    vendor.inlet.shop/password/{id}/{token}
</a>

<p>-Inlet Sites</p>
"#
    )
}
