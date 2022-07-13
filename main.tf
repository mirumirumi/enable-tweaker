provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  
provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  
provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  
provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  
provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}


  resource "aws_s3_bucket" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "tfstate" {
  bucket = "${var.resource_prefix}-${var.env_name}-tfstate-artifactstore"
  acl    = "private"
}

resource "aws_s3_bucket" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"

  tags = var.tags
}

resource "aws_s3_bucket_acl" "sam-artifactstore" {
  bucket = "${var.resource_prefix}-${var.env_name}-sam-artifactstore"
  acl    = "private"
}

  variable "resource_prefix" {
}

variable "env_name" {
}

variable "tags" {
}

  provider "aws" {
  region = "ap-northeast-1"
}

terraform {
  backend "s3" {
    bucket = "common-dev-tfstate-artifactstore"
    region = "ap-northeast-1"
    key    = "terraform.tfstate"
  }
}

locals {
  resource_prefix = "common"
  env_name        = "dev"
}

locals {
  tags = {
    project = "common"
    env     = local.env_name
    IaC     = "tf"
  }
}

module "modules" {
  source   = "../../modules/common"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}

module "virginia" {
  source   = "../../modules/virginia"

  resource_prefix = local.resource_prefix
  env_name        = local.env_name
  tags            = local.tags
}
