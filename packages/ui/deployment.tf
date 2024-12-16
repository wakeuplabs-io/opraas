# variables ==================================================================

variable "customer" {
  description = "Customer for tag label"
  type        = string
  default     = "op-ruaas"
}

variable "bucket_name" {
  description = "The name of the S3 bucket"
  type        = string
  default     = "op-ruaas"
}

# resources ==================================================================

# create bucket
resource "aws_s3_bucket" "site_origin" {
  bucket = var.bucket_name
  tags = {
    customer = "${var.customer}"
  }
}

# assign policy to allow CloudFront to reach S3 bucket
resource "aws_s3_bucket_policy" "origin" {
  depends_on = [
    aws_cloudfront_distribution.site_access
  ]
  bucket = aws_s3_bucket.site_origin.id
  policy = data.aws_iam_policy_document.origin.json
}

# create policy to allow CloudFront to reach S3 bucket
data "aws_iam_policy_document" "origin" {
  depends_on = [
    aws_cloudfront_distribution.site_access,
    aws_s3_bucket.site_origin
  ]
  statement {
    sid    = "3"
    effect = "Allow"
    actions = [
      "s3:GetObject"
    ]
    principals {
      identifiers = ["cloudfront.amazonaws.com"]
      type        = "Service"
    }
    resources = [
      "arn:aws:s3:::${aws_s3_bucket.site_origin.bucket}/*"
    ]
    condition {
      test     = "StringEquals"
      variable = "AWS:SourceArn"

      values = [
        aws_cloudfront_distribution.site_access.arn
      ]
    }
  }
}

# enable AWS S3 file versioning
resource "aws_s3_bucket_versioning" "site_origin" {
  bucket = aws_s3_bucket.site_origin.bucket
  versioning_configuration {
    status = "Enabled"
  }
}

# create CloudFront distribution group
resource "aws_cloudfront_distribution" "site_access" {
  depends_on = [
    aws_s3_bucket.site_origin,
    aws_cloudfront_origin_access_control.site_access
  ]

  origin {
    domain_name              = aws_s3_bucket.site_origin.bucket_regional_domain_name
    origin_id                = aws_s3_bucket.site_origin.id
    origin_access_control_id = aws_cloudfront_origin_access_control.site_access.id
  }

  enabled             = true
  default_root_object = "index.html"

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  default_cache_behavior {
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = aws_s3_bucket.site_origin.id
    viewer_protocol_policy = "https-only"

    forwarded_values {
      query_string = false

      cookies {
        forward = "none"
      }

    }
  }

  viewer_certificate {
    cloudfront_default_certificate = true
  }

  tags = {
    customer = "${var.customer}"
  }
}


# create Origin Access Control as this is required to allow access to the s3 bucket without public access to the S3 bucket.
resource "aws_cloudfront_origin_access_control" "site_access" {
  name                              = "Security_Pillar100_CF_S3_OAC"
  description                       = "OAC setup for security pillar 100"
  origin_access_control_origin_type = "s3"
  signing_behavior                  = "always"
  signing_protocol                  = "sigv4"
}

# outputs ==================================================================

output "bucket_name" {
  value = aws_s3_bucket.site_origin.bucket
}

output "cloudfront_url" {
  value = aws_cloudfront_distribution.site_access.domain_name
}

output "s3_sync" {
  description = "S3 sync command. Run for each deployment, even after `terraform apply`"
  value       = "aws s3 sync dist s3://${aws_s3_bucket.site_origin.bucket} --delete"
}

output "invalidate_cloudfront" {
  description = "Cloudfront invalidation command. Run after each s3 sync command"
  value       = "aws cloudfront create-invalidation --distribution-id ${aws_cloudfront_distribution.site_access.id} --paths '/*'"
}
